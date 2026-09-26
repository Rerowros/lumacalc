use std::fmt;
use std::path::Path;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HotkeyError {
    pub message: String,
}

impl fmt::Display for HotkeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for HotkeyError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StartupError {
    pub message: String,
}

impl fmt::Display for StartupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for StartupError {}

const STARTUP_VALUE_NAME: &str = "LumaCalc";

/// Returns the exact per-user startup command owned by LumaCalc.
pub fn startup_command(executable: &Path) -> String {
    format!("\"{}\" --background", executable.to_string_lossy())
}

/// Reports whether the owned per-user startup entry points at this executable.
pub fn startup_enabled(executable: &Path) -> Result<bool, StartupError> {
    startup_enabled_impl(executable)
}

/// Creates or removes only LumaCalc's per-user startup value.
pub fn set_startup_enabled(executable: &Path, enabled: bool) -> Result<(), StartupError> {
    set_startup_enabled_impl(executable, enabled)
}

pub struct GlobalHotkey {
    #[cfg(windows)]
    thread_id: u32,
    join: Option<JoinHandle<()>>,
}

impl GlobalHotkey {
    /// Registers Win+Alt+Space and calls `on_pressed` from the hotkey thread.
    /// The callback should post work to the UI event loop and return quickly.
    pub fn register(on_pressed: impl Fn() + Send + 'static) -> Result<Self, HotkeyError> {
        register_impl(on_pressed)
    }
}

impl Drop for GlobalHotkey {
    fn drop(&mut self) {
        #[cfg(windows)]
        unsafe {
            use windows::Win32::Foundation::{LPARAM, WPARAM};
            use windows::Win32::UI::WindowsAndMessaging::{PostThreadMessageW, WM_QUIT};

            let _ = PostThreadMessageW(self.thread_id, WM_QUIT, WPARAM(0), LPARAM(0));
        }

        if let Some(join) = self.join.take() {
            let _ = join.join();
        }
    }
}

#[cfg(windows)]
fn register_impl(on_pressed: impl Fn() + Send + 'static) -> Result<GlobalHotkey, HotkeyError> {
    use windows::Win32::System::Threading::GetCurrentThreadId;
    use windows::Win32::UI::Input::KeyboardAndMouse::{
        MOD_ALT, MOD_NOREPEAT, MOD_WIN, RegisterHotKey, UnregisterHotKey, VK_SPACE,
    };
    use windows::Win32::UI::WindowsAndMessaging::{GetMessageW, MSG, WM_HOTKEY};

    const HOTKEY_ID: i32 = 0x4C43; // "LC"

    let (ready_tx, ready_rx) = mpsc::sync_channel(1);
    let join = thread::Builder::new()
        .name("lumacalc-hotkey".to_owned())
        .spawn(move || unsafe {
            let thread_id = GetCurrentThreadId();
            let registration = RegisterHotKey(
                None,
                HOTKEY_ID,
                MOD_WIN | MOD_ALT | MOD_NOREPEAT,
                u32::from(VK_SPACE.0),
            );

            match registration {
                Ok(()) => {
                    let _ = ready_tx.send(Ok(thread_id));
                }
                Err(err) => {
                    let _ = ready_tx.send(Err(HotkeyError {
                        message: format!(
                            "Win+Alt+Space уже используется другим приложением ({err})"
                        ),
                    }));
                    return;
                }
            }

            let mut message = MSG::default();
            loop {
                let status = GetMessageW(&mut message, None, 0, 0).0;
                if status <= 0 {
                    break;
                }
                if message.message == WM_HOTKEY && message.wParam.0 == HOTKEY_ID as usize {
                    on_pressed();
                }
            }
            let _ = UnregisterHotKey(None, HOTKEY_ID);
        })
        .map_err(|err| HotkeyError {
            message: format!("Не удалось запустить обработчик горячей клавиши: {err}"),
        })?;

    match ready_rx.recv() {
        Ok(Ok(thread_id)) => Ok(GlobalHotkey {
            thread_id,
            join: Some(join),
        }),
        Ok(Err(err)) => {
            let _ = join.join();
            Err(err)
        }
        Err(err) => {
            let _ = join.join();
            Err(HotkeyError {
                message: format!("Обработчик горячей клавиши не ответил: {err}"),
            })
        }
    }
}

#[cfg(not(windows))]
fn register_impl(_on_pressed: impl Fn() + Send + 'static) -> Result<GlobalHotkey, HotkeyError> {
    Err(HotkeyError {
        message: "Глобальный overlay поддерживается только в Windows".to_owned(),
    })
}

#[cfg(windows)]
fn startup_enabled_impl(executable: &Path) -> Result<bool, StartupError> {
    use windows_registry::CURRENT_USER;

    const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
    let Ok(key) = CURRENT_USER.open(RUN_KEY) else {
        return Ok(false);
    };
    let Ok(command) = key.get_string(STARTUP_VALUE_NAME) else {
        return Ok(false);
    };
    Ok(command == startup_command(executable))
}

#[cfg(windows)]
fn set_startup_enabled_impl(executable: &Path, enabled: bool) -> Result<(), StartupError> {
    use windows_registry::CURRENT_USER;

    const RUN_KEY: &str = r"Software\Microsoft\Windows\CurrentVersion\Run";
    if enabled {
        let key = CURRENT_USER.create(RUN_KEY).map_err(|error| StartupError {
            message: format!("Не удалось изменить автозапуск LumaCalc: {error}"),
        })?;
        key.set_string(STARTUP_VALUE_NAME, startup_command(executable))
            .map_err(|error| StartupError {
                message: format!("Не удалось изменить автозапуск LumaCalc: {error}"),
            })
    } else {
        let Ok(key) = CURRENT_USER.options().read().write().open(RUN_KEY) else {
            return Ok(());
        };
        if key.get_string(STARTUP_VALUE_NAME).is_ok() {
            key.remove_value(STARTUP_VALUE_NAME)
                .map_err(|error| StartupError {
                    message: format!("Не удалось изменить автозапуск LumaCalc: {error}"),
                })?;
        }
        Ok(())
    }
}

#[cfg(not(windows))]
fn startup_enabled_impl(_executable: &Path) -> Result<bool, StartupError> {
    Ok(false)
}

#[cfg(not(windows))]
fn set_startup_enabled_impl(_executable: &Path, _enabled: bool) -> Result<(), StartupError> {
    Err(StartupError {
        message: "Автозапуск поддерживается только в Windows".to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_is_user_readable() {
        let error = HotkeyError {
            message: "test".to_owned(),
        };
        assert_eq!(error.to_string(), "test");
    }

    #[test]
    fn startup_command_quotes_paths_with_spaces() {
        let command = startup_command(Path::new(r"C:\Program Files\LumaCalc\LumaCalc.exe"));
        assert_eq!(
            command,
            r#""C:\Program Files\LumaCalc\LumaCalc.exe" --background"#
        );
    }
}
