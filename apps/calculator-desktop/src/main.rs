#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use calc_engine::{CalculatorEngine, DetailTone, Evaluation};
use calc_platform_windows::{GlobalHotkey, set_startup_enabled, startup_enabled};
use slint::winit_030::{WinitWindowAccessor, winit};
use slint::{ComponentHandle, ModelRc, SharedString, VecModel};

slint::include_modules!();

const APP_NAME: &str = "LumaCalc";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    configure_backend()?;

    let background = std::env::args_os().any(|argument| argument == "--background");
    let executable = std::env::current_exe()?;

    let app = AppWindow::new()?;
    let engine = Rc::new(RefCell::new(CalculatorEngine::new()));
    let history = Rc::new(RefCell::new(Vec::<HistoryRow>::new()));
    let visible = Arc::new(AtomicBool::new(!background));

    app.set_app_name(APP_NAME.into());
    app.set_overlay_mode(background);
    app.set_result("Готов к вычислениям".into());
    app.set_result_caption("Введите выражение, например 15 мб".into());
    app.set_hotkey_status("Win + Alt + Space".into());
    set_conversion_rows(&app, &[]);
    set_history_rows(&app, &[]);
    match startup_enabled(&executable) {
        Ok(enabled) => {
            app.set_startup_enabled(enabled);
            app.set_startup_message(
                if enabled {
                    "Включено · фоновый запуск после входа в Windows"
                } else {
                    "Выключено · LumaCalc запускается только вручную"
                }
                .into(),
            );
        }
        Err(error) => app.set_startup_message(error.to_string().into()),
    }

    wire_evaluation(&app, Rc::clone(&engine), Rc::clone(&history));
    wire_window_actions(&app, &visible, executable);

    if !background {
        set_full_window(&app);
        app.show()?;
        app.invoke_focus_expression();
    }

    let hotkey = register_hotkey(&app, Arc::clone(&visible));
    if let Err(error) = &hotkey {
        app.set_hotkey_status(format!("Hotkey недоступен: {error}").into());
        if background {
            set_full_window(&app);
            app.window().show()?;
            visible.store(true, Ordering::Release);
        }
    }

    slint::run_event_loop_until_quit()?;
    drop(hotkey);
    Ok(())
}

fn configure_backend() -> Result<(), slint::PlatformError> {
    use winit::window::WindowLevel;

    slint::BackendSelector::new()
        .backend_name("winit".into())
        .renderer_name("software".into())
        .with_winit_window_attributes_hook(|attributes| {
            attributes
                .with_decorations(false)
                .with_resizable(false)
                .with_transparent(true)
                .with_window_level(WindowLevel::AlwaysOnTop)
                .with_title(APP_NAME)
        })
        .select()
}

fn wire_evaluation(
    app: &AppWindow,
    engine: Rc<RefCell<CalculatorEngine>>,
    history: Rc<RefCell<Vec<HistoryRow>>>,
) {
    let weak = app.as_weak();
    app.on_evaluate(move |expression| {
        let Some(app) = weak.upgrade() else {
            return;
        };

        match engine.borrow_mut().evaluate(expression.as_str()) {
            Ok(evaluation) => {
                show_evaluation(&app, &evaluation);
                let mut history = history.borrow_mut();
                history.insert(
                    0,
                    HistoryRow {
                        expression: evaluation.source.clone().into(),
                        result: evaluation.result.clone().into(),
                    },
                );
                history.truncate(30);
                set_history_rows(&app, &history);
                app.set_has_error(false);
                app.set_error_text(SharedString::default());
            }
            Err(error) => {
                app.set_has_error(true);
                app.set_error_text(error.message.into());
                app.set_result("—".into());
                app.set_result_caption(error.code.into());
                set_conversion_rows(&app, &[]);
            }
        }
    });

    let weak = app.as_weak();
    app.on_use_example(move |example| {
        if let Some(app) = weak.upgrade() {
            app.set_expression(example);
            app.invoke_focus_expression();
        }
    });

    let weak = app.as_weak();
    app.on_recall_history(move |expression| {
        if let Some(app) = weak.upgrade() {
            app.set_expression(expression);
            app.invoke_focus_expression();
        }
    });
}

fn wire_window_actions(app: &AppWindow, visible: &Arc<AtomicBool>, executable: std::path::PathBuf) {
    let weak = app.as_weak();
    let hidden = Arc::clone(visible);
    app.on_hide_requested(move || {
        if let Some(app) = weak.upgrade() {
            let _ = app.window().hide();
            hidden.store(false, Ordering::Release);
        }
    });

    let weak = app.as_weak();
    app.on_expand_requested(move || {
        if let Some(app) = weak.upgrade() {
            set_full_window(&app);
            app.invoke_focus_expression();
        }
    });

    let weak = app.as_weak();
    app.on_compact_requested(move || {
        if let Some(app) = weak.upgrade() {
            app.set_overlay_mode(true);
            app.window().with_winit_window(|window| {
                window.set_window_level(winit::window::WindowLevel::AlwaysOnTop);
                window.set_resizable(false);
            });
            app.invoke_focus_expression();
        }
    });

    let weak = app.as_weak();
    app.on_drag_requested(move || {
        if let Some(app) = weak.upgrade() {
            app.window().with_winit_window(|window| {
                let _ = window.drag_window();
            });
        }
    });

    app.on_quit_requested(|| {
        let _ = slint::quit_event_loop();
    });

    let weak = app.as_weak();
    app.on_set_startup_enabled(move |enabled| {
        let Some(app) = weak.upgrade() else {
            return;
        };
        match set_startup_enabled(&executable, enabled) {
            Ok(()) => {
                app.set_startup_enabled(enabled);
                app.set_startup_message(
                    if enabled {
                        "Включено · LumaCalc будет ждать Win + Alt + Space"
                    } else {
                        "Выключено · запись автозапуска удалена"
                    }
                    .into(),
                );
            }
            Err(error) => {
                app.set_startup_message(error.to_string().into());
                if let Ok(actual) = startup_enabled(&executable) {
                    app.set_startup_enabled(actual);
                }
            }
        }
    });
}

fn set_full_window(app: &AppWindow) {
    app.set_overlay_mode(false);
    app.window().with_winit_window(|window| {
        window.set_window_level(winit::window::WindowLevel::Normal);
        window.set_resizable(true);
    });
}

fn register_hotkey(
    app: &AppWindow,
    visible: Arc<AtomicBool>,
) -> Result<GlobalHotkey, calc_platform_windows::HotkeyError> {
    let weak = app.as_weak();
    GlobalHotkey::register(move || {
        let weak = weak.clone();
        let visible = Arc::clone(&visible);
        let _ = slint::invoke_from_event_loop(move || {
            let Some(app) = weak.upgrade() else {
                return;
            };
            if visible.load(Ordering::Acquire) && app.get_overlay_mode() {
                let _ = app.window().hide();
                visible.store(false, Ordering::Release);
            } else {
                app.set_overlay_mode(true);
                app.window().with_winit_window(|window| {
                    window.set_window_level(winit::window::WindowLevel::AlwaysOnTop);
                    window.focus_window();
                });
                if app.window().show().is_ok() {
                    visible.store(true, Ordering::Release);
                    app.invoke_focus_expression();
                }
            }
        });
    })
}

fn show_evaluation(app: &AppWindow, evaluation: &Evaluation) {
    app.set_result(evaluation.result.clone().into());
    app.set_result_caption(evaluation.caption.clone().into());
    app.set_approximate(evaluation.approximate);
    set_conversion_rows(app, &evaluation.details);
}

fn set_conversion_rows(app: &AppWindow, details: &[calc_engine::DetailRow]) {
    let rows = details
        .iter()
        .map(|detail| ConversionRow {
            label: detail.label.clone().into(),
            value: detail.value.clone().into(),
            tone: match detail.tone {
                DetailTone::Neutral => "neutral",
                DetailTone::Accent => "accent",
                DetailTone::Binary => "binary",
            }
            .into(),
        })
        .collect::<Vec<_>>();
    app.set_conversions(ModelRc::from(Rc::new(VecModel::from(rows))));
}

fn set_history_rows(app: &AppWindow, rows: &[HistoryRow]) {
    app.set_history(ModelRc::from(Rc::new(VecModel::from(rows.to_vec()))));
}
