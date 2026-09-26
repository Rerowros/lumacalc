# User-level installation

Build the release executable first:

```powershell
cargo build --release -p calculator-desktop
```

Install without elevation and without implicit startup registration:

```powershell
.\packaging\install-user.ps1
```

To explicitly enable launch at sign-in during installation:

```powershell
.\packaging\install-user.ps1 -EnableStartup
```

The installer copies `LumaCalc.exe` to `%LOCALAPPDATA%\Programs\LumaCalc`, creates a Start Menu shortcut, and changes no machine-wide state. The application Settings page can enable or disable its per-user `HKCU` startup value later.

Uninstall:

```powershell
.\packaging\uninstall-user.ps1
```

The uninstall script removes only the installed executable, its shortcut, and an exact matching `LumaCalc` startup entry. User calculation data is preserved.
