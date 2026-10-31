//! MagicBridge Unified Executable (runs daemon, installer, uninstaller, or settings GUI).
//!
//! Complies with single-binary UX requirements:
//! - `--daemon`: Runs headless background hook and IPC daemon (zero window).
//! - `--uninstall`: Runs native GUI uninstaller wizard.
//! - `--install` / run outside install dir: Runs native GUI installer wizard.
//! - Default / `--settings`: Runs native Settings GUI (with auto daemon spawn and onboarding gate).

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{NativeOptions, egui::ViewportBuilder};
use magicbridge_settings::{
    MagicBridgeApp, is_installed_location, run_installer_gui, run_uninstaller_gui,
};

fn main() -> Result<(), eframe::Error> {
    let args: Vec<String> = std::env::args().collect();

    // 1. Daemon mode: runs the background hook daemon
    if args.iter().any(|a| a == "--daemon") {
        let code = magicbridge_agent::run_daemon();
        std::process::exit(code);
    }

    // 2. Uninstaller mode
    if args.iter().any(|a| a == "--uninstall") {
        return run_uninstaller_gui();
    }

    // 3. Installer mode check:
    let is_installed = is_installed_location();
    let is_explicit_settings = args.iter().any(|a| a == "--settings");
    let is_explicit_install = args.iter().any(|a| a == "--install");
    let is_dev_or_test = std::env::current_exe()
        .map(|p| {
            let s = p.to_string_lossy().to_lowercase();
            s.contains(r"\target\") || s.contains("ui-qa")
        })
        .unwrap_or(false);

    if is_explicit_install || (!is_installed && !is_explicit_settings && !is_dev_or_test) {
        return run_installer_gui();
    }

    // 4. Normal Settings GUI mode:
    // Ensure background daemon is running
    magicbridge_settings::installer::ensure_daemon_running();

    let native_options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([960.0, 580.0])
            .with_min_inner_size([860.0, 480.0])
            .with_title("MagicBridge Settings"),
        ..Default::default()
    };

    eframe::run_native(
        "MagicBridge Settings",
        native_options,
        Box::new(|cc| Ok(Box::new(MagicBridgeApp::new(cc)))),
    )
}
