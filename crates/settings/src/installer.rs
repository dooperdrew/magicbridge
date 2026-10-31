//! Native Windows GUI Installer, Uninstaller, and Process Manager for MagicBridge.
//!
//! Provides zero-elevation (per-user) installation into `%LOCALAPPDATA%\Programs\MagicBridge`,
//! Start Menu and Desktop shortcuts, Windows Settings "Installed apps" registration,
//! clean uninstallation, and background daemon orchestration.

use std::path::{Path, PathBuf};
use std::process::Command;

use eframe::{
    App, NativeOptions,
    egui::{self, Color32, CornerRadius, RichText, ViewportBuilder},
};
use windows::Win32::Foundation::{ERROR_SUCCESS, WIN32_ERROR};
use windows::Win32::System::Registry::{
    HKEY_CURRENT_USER, KEY_SET_VALUE, REG_SZ, RegCloseKey, RegCreateKeyW, RegDeleteKeyW,
    RegDeleteValueW, RegOpenKeyExW, RegSetValueExW,
};
use windows::core::PCWSTR;

/// Default installation directory under `%LOCALAPPDATA%`.
pub fn default_install_dir() -> PathBuf {
    if let Ok(local_appdata) = std::env::var("LOCALAPPDATA") {
        PathBuf::from(local_appdata)
            .join("Programs")
            .join("MagicBridge")
    } else {
        PathBuf::from(r"C:\Program Files\MagicBridge")
    }
}

/// Returns true if the currently running executable is executing from the installed location.
#[must_use]
pub fn is_installed_location() -> bool {
    let current = match std::env::current_exe() {
        Ok(p) => p,
        Err(_) => return false,
    };

    let target_dir = default_install_dir();
    current.starts_with(&target_dir)
}

/// Checks if the background daemon is alive; if not, launches it via `--daemon`.
pub fn ensure_daemon_running() {
    let ipc = crate::ipc::SettingsIpc::new();
    if ipc.query_status().is_some() {
        return;
    }

    if let Ok(exe_path) = std::env::current_exe() {
        let mut cmd = Command::new(&exe_path);
        cmd.arg("--daemon");
        let _ = cmd.spawn();
    }
}

/// Options selected in the installer wizard GUI.
#[derive(Debug, Clone)]
pub struct InstallOptions {
    pub install_dir: PathBuf,
    pub create_desktop_shortcut: bool,
    pub create_start_menu_shortcut: bool,
    pub autostart_on_login: bool,
    pub launch_on_finish: bool,
}

impl Default for InstallOptions {
    fn default() -> Self {
        Self {
            install_dir: default_install_dir(),
            create_desktop_shortcut: true,
            create_start_menu_shortcut: true,
            autostart_on_login: true,
            launch_on_finish: true,
        }
    }
}

/// Creates a Windows `.lnk` shortcut file pointing to the target executable.
pub fn create_shortcut(
    target_exe: &Path,
    shortcut_path: &Path,
    description: &str,
) -> Result<(), String> {
    if let Some(parent) = shortcut_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let ps_cmd = format!(
        "$ws = New-Object -ComObject WScript.Shell; $s = $ws.CreateShortcut('{}'); $s.TargetPath = '{}'; $s.Description = '{}'; $s.WorkingDirectory = '{}'; $s.Save()",
        shortcut_path.display(),
        target_exe.display(),
        description,
        target_exe
            .parent()
            .map(|p| p.display().to_string())
            .unwrap_or_default()
    );

    let status = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &ps_cmd])
        .status()
        .map_err(|e| format!("Failed to run PowerShell shortcut creator: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "PowerShell shortcut creation exited with code {:?}",
            status.code()
        ))
    }
}

/// Executes installation into the destination folder and registers in Windows.
pub fn perform_install(options: &InstallOptions) -> Result<(), String> {
    let current_exe =
        std::env::current_exe().map_err(|e| format!("Cannot locate current exe: {e}"))?;
    std::fs::create_dir_all(&options.install_dir)
        .map_err(|e| format!("Failed to create install directory: {e}"))?;

    let dest_exe = options.install_dir.join("magicbridge.exe");

    // Terminate running instances before overwrite if present
    if dest_exe.exists() {
        let _ = Command::new("taskkill")
            .args(["/F", "/IM", "magicbridge.exe"])
            .status();
        std::thread::sleep(std::time::Duration::from_millis(300));
    }

    // Copy executable
    std::fs::copy(&current_exe, &dest_exe)
        .map_err(|e| format!("Failed to copy binary to destination: {e}"))?;

    // Create Start Menu Shortcut
    if options.create_start_menu_shortcut
        && let Ok(appdata) = std::env::var("APPDATA")
    {
        let start_menu_path = PathBuf::from(appdata)
            .join(r"Microsoft\Windows\Start Menu\Programs")
            .join("MagicBridge.lnk");
        let _ = create_shortcut(
            &dest_exe,
            &start_menu_path,
            "MagicBridge for Apple Keyboard",
        );
    }

    // Create Desktop Shortcut
    if options.create_desktop_shortcut
        && let Ok(userprofile) = std::env::var("USERPROFILE")
    {
        let desktop_path = PathBuf::from(userprofile)
            .join("Desktop")
            .join("MagicBridge.lnk");
        let _ = create_shortcut(&dest_exe, &desktop_path, "MagicBridge for Apple Keyboard");
    }

    // Autostart in HKCU Run
    if options.autostart_on_login {
        let dest_str = format!("\"{}\" --daemon", dest_exe.display());
        let _ = write_hkcu_string(
            r"Software\Microsoft\Windows\CurrentVersion\Run",
            "MagicBridge",
            &dest_str,
        );
    }

    // Register in Windows Settings "Installed apps"
    let uninstall_key = r"Software\Microsoft\Windows\CurrentVersion\Uninstall\MagicBridge";
    let uninstall_str = format!("\"{}\" --uninstall", dest_exe.display());
    let icon_str = format!("\"{}\",0", dest_exe.display());

    let _ = write_hkcu_string(
        uninstall_key,
        "DisplayName",
        "MagicBridge (Apple Magic Keyboard for Windows)",
    );
    let _ = write_hkcu_string(uninstall_key, "DisplayVersion", "0.3.0");
    let _ = write_hkcu_string(uninstall_key, "Publisher", "MagicBridge Contributors");
    let _ = write_hkcu_string(
        uninstall_key,
        "InstallLocation",
        &options.install_dir.display().to_string(),
    );
    let _ = write_hkcu_string(uninstall_key, "UninstallString", &uninstall_str);
    let _ = write_hkcu_string(uninstall_key, "DisplayIcon", &icon_str);

    // Launch installed app if requested
    if options.launch_on_finish {
        let _ = Command::new(&dest_exe).spawn();
    }

    Ok(())
}

/// Performs clean uninstallation of MagicBridge.
pub fn perform_uninstall(remove_user_data: bool) -> Result<(), String> {
    // 1. Terminate running instances
    let _ = Command::new("taskkill")
        .args(["/F", "/IM", "magicbridge.exe"])
        .status();

    // 2. Remove autostart entry
    let _ = delete_hkcu_value(
        r"Software\Microsoft\Windows\CurrentVersion\Run",
        "MagicBridge",
    );

    // 3. Remove uninstall entry
    let _ = delete_hkcu_key(r"Software\Microsoft\Windows\CurrentVersion\Uninstall\MagicBridge");

    // 4. Remove Shortcuts
    if let Ok(appdata) = std::env::var("APPDATA") {
        let start_menu_path = PathBuf::from(appdata)
            .join(r"Microsoft\Windows\Start Menu\Programs")
            .join("MagicBridge.lnk");
        let _ = std::fs::remove_file(start_menu_path);
    }
    if let Ok(userprofile) = std::env::var("USERPROFILE") {
        let desktop_path = PathBuf::from(userprofile)
            .join("Desktop")
            .join("MagicBridge.lnk");
        let _ = std::fs::remove_file(desktop_path);
    }

    // 5. Remove user configuration if requested
    if remove_user_data && let Ok(config_dir) = magicbridge_core::default_config_dir() {
        let _ = std::fs::remove_dir_all(config_dir);
    }

    // 6. Remove installation directory
    let install_dir = default_install_dir();
    if install_dir.exists() {
        // Schedule cleanup batch command because the currently running uninstaller may reside inside
        let ps_cmd = format!(
            "Start-Sleep -Seconds 1; Remove-Item -Path '{}' -Recurse -Force",
            install_dir.display()
        );
        let _ = Command::new("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", &ps_cmd])
            .spawn();
    }

    Ok(())
}

// ----------------------------------------------------------------------------
// Win32 Registry Helpers
// ----------------------------------------------------------------------------

fn encode_wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn write_hkcu_string(sub_key: &str, value_name: &str, data: &str) -> Result<(), WIN32_ERROR> {
    let sub_key_w = encode_wide(sub_key);
    let value_name_w = encode_wide(value_name);
    let data_w = encode_wide(data);

    let mut hkey = windows::Win32::System::Registry::HKEY::default();
    // SAFETY: RegCreateKeyW opens or creates sub_key in HKCU.
    let status = unsafe {
        RegCreateKeyW(
            HKEY_CURRENT_USER,
            PCWSTR::from_raw(sub_key_w.as_ptr()),
            &mut hkey,
        )
    };
    if status != ERROR_SUCCESS {
        return Err(status);
    }

    // SAFETY: RegSetValueExW sets string value data.
    let byte_count = (data_w.len() * std::mem::size_of::<u16>()) as u32;
    // SAFETY: data_w is a valid null-terminated UTF-16 buffer with byte length byte_count.
    let byte_slice =
        unsafe { std::slice::from_raw_parts(data_w.as_ptr() as *const u8, byte_count as usize) };
    // SAFETY: hkey is an open HKCU registry key handle with KEY_SET_VALUE access permissions.
    let set_status = unsafe {
        RegSetValueExW(
            hkey,
            PCWSTR::from_raw(value_name_w.as_ptr()),
            None,
            REG_SZ,
            Some(byte_slice),
        )
    };

    // SAFETY: RegCloseKey closes the opened registry key.
    unsafe {
        let _ = RegCloseKey(hkey);
    }

    if set_status != ERROR_SUCCESS {
        Err(set_status)
    } else {
        Ok(())
    }
}

fn delete_hkcu_value(sub_key: &str, value_name: &str) -> Result<(), WIN32_ERROR> {
    let sub_key_w = encode_wide(sub_key);
    let value_name_w = encode_wide(value_name);

    let mut hkey = windows::Win32::System::Registry::HKEY::default();
    // SAFETY: RegOpenKeyExW opens subkey with KEY_SET_VALUE.
    let status = unsafe {
        RegOpenKeyExW(
            HKEY_CURRENT_USER,
            PCWSTR::from_raw(sub_key_w.as_ptr()),
            None,
            KEY_SET_VALUE,
            &mut hkey,
        )
    };
    if status != ERROR_SUCCESS {
        return Err(status);
    }

    // SAFETY: RegDeleteValueW removes the named value.
    let del_status = unsafe { RegDeleteValueW(hkey, PCWSTR::from_raw(value_name_w.as_ptr())) };

    // SAFETY: RegCloseKey closes registry handle.
    unsafe {
        let _ = RegCloseKey(hkey);
    }

    if del_status != ERROR_SUCCESS {
        Err(del_status)
    } else {
        Ok(())
    }
}

fn delete_hkcu_key(sub_key: &str) -> Result<(), WIN32_ERROR> {
    let sub_key_w = encode_wide(sub_key);
    // SAFETY: RegDeleteKeyW deletes the entire subkey from HKCU.
    let status = unsafe { RegDeleteKeyW(HKEY_CURRENT_USER, PCWSTR::from_raw(sub_key_w.as_ptr())) };
    if status != ERROR_SUCCESS {
        Err(status)
    } else {
        Ok(())
    }
}

// ----------------------------------------------------------------------------
// GUI Installer & Uninstaller Applications
// ----------------------------------------------------------------------------

/// Standalone GUI Installer wizard application.
#[derive(Debug, Clone, Default)]
pub struct InstallerApp {
    pub options: InstallOptions,
    pub status_message: Option<Result<String, String>>,
    pub is_finished: bool,
}

impl App for InstallerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);
            ui.heading("⌨ MagicBridge Setup");
            ui.label("Apple Magic Keyboard with Numeric Keypad (A1843) for Windows 10/11");
            ui.add_space(10.0);

            if self.is_finished {
                ui.colored_label(
                    Color32::from_rgb(40, 160, 60),
                    RichText::new("✔ Installation completed successfully!")
                        .size(16.0)
                        .strong(),
                );
                ui.add_space(10.0);
                ui.label("MagicBridge is now installed and configured on your computer.");
                ui.add_space(16.0);
                if ui.button(RichText::new("Close").size(14.0)).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                return;
            }

            ui.group(|ui| {
                ui.label(RichText::new("Installation Location:").strong());
                ui.label(self.options.install_dir.display().to_string());
            });

            ui.add_space(8.0);
            ui.group(|ui| {
                ui.label(RichText::new("Setup Options:").strong());
                ui.checkbox(
                    &mut self.options.create_desktop_shortcut,
                    "Create Desktop Shortcut",
                );
                ui.checkbox(
                    &mut self.options.create_start_menu_shortcut,
                    "Create Start Menu Shortcut",
                );
                ui.checkbox(
                    &mut self.options.autostart_on_login,
                    "Start MagicBridge automatically at Windows logon",
                );
                ui.checkbox(
                    &mut self.options.launch_on_finish,
                    "Launch MagicBridge immediately after setup",
                );
            });

            if let Some(ref res) = self.status_message {
                ui.add_space(8.0);
                match res {
                    Ok(msg) => {
                        ui.colored_label(Color32::from_rgb(40, 160, 60), msg);
                    }
                    Err(err) => {
                        ui.colored_label(Color32::from_rgb(220, 50, 40), format!("Error: {err}"));
                    }
                }
            }

            ui.add_space(16.0);
            ui.horizontal(|ui| {
                let install_btn = egui::Button::new(
                    RichText::new("Install Now")
                        .size(14.0)
                        .strong()
                        .color(Color32::WHITE),
                )
                .fill(Color32::from_rgb(0, 120, 215))
                .corner_radius(CornerRadius::same(6));

                if ui.add(install_btn).clicked() {
                    match perform_install(&self.options) {
                        Ok(()) => {
                            self.is_finished = true;
                            self.status_message = Some(Ok("Installed successfully!".to_string()));
                        }
                        Err(e) => {
                            self.status_message = Some(Err(e));
                        }
                    }
                }

                if ui.button("Cancel").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}

/// Standalone GUI Uninstaller application.
#[derive(Debug, Clone, Default)]
pub struct UninstallerApp {
    pub remove_user_data: bool,
    pub status_message: Option<Result<String, String>>,
    pub is_finished: bool,
}

impl App for UninstallerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(8.0);
            ui.heading("🗑 MagicBridge Uninstaller");
            ui.add_space(8.0);

            if self.is_finished {
                ui.colored_label(
                    Color32::from_rgb(40, 160, 60),
                    RichText::new("✔ MagicBridge was successfully removed.")
                        .size(16.0)
                        .strong(),
                );
                ui.add_space(16.0);
                if ui.button(RichText::new("Close").size(14.0)).clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                return;
            }

            ui.label(
                "This will completely remove MagicBridge and all its shortcuts from your computer.",
            );
            ui.add_space(8.0);
            ui.checkbox(
                &mut self.remove_user_data,
                "Also remove custom key profiles and user configuration files",
            );

            if let Some(ref res) = self.status_message {
                ui.add_space(8.0);
                match res {
                    Ok(msg) => {
                        ui.colored_label(Color32::from_rgb(40, 160, 60), msg);
                    }
                    Err(err) => {
                        ui.colored_label(Color32::from_rgb(220, 50, 40), format!("Error: {err}"));
                    }
                }
            }

            ui.add_space(16.0);
            ui.horizontal(|ui| {
                let uninstall_btn = egui::Button::new(
                    RichText::new("Uninstall Now")
                        .size(14.0)
                        .strong()
                        .color(Color32::WHITE),
                )
                .fill(Color32::from_rgb(210, 45, 40))
                .corner_radius(CornerRadius::same(6));

                if ui.add(uninstall_btn).clicked() {
                    match perform_uninstall(self.remove_user_data) {
                        Ok(()) => {
                            self.is_finished = true;
                            self.status_message = Some(Ok("Uninstalled successfully!".to_string()));
                        }
                        Err(e) => {
                            self.status_message = Some(Err(e));
                        }
                    }
                }

                if ui.button("Cancel").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        });
    }
}

/// Runs the native GUI Installer wizard window.
pub fn run_installer_gui() -> Result<(), eframe::Error> {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([580.0, 380.0])
            .with_resizable(false)
            .with_title("MagicBridge Setup"),
        ..Default::default()
    };
    eframe::run_native(
        "MagicBridge Setup",
        native_options,
        Box::new(|_cc| Ok(Box::new(InstallerApp::default()))),
    )
}

/// Runs the native GUI Uninstaller window.
pub fn run_uninstaller_gui() -> Result<(), eframe::Error> {
    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([520.0, 300.0])
            .with_resizable(false)
            .with_title("MagicBridge Uninstaller"),
        ..Default::default()
    };
    eframe::run_native(
        "MagicBridge Uninstaller",
        native_options,
        Box::new(|_cc| Ok(Box::new(UninstallerApp::default()))),
    )
}
