//! About view: version, daemon diagnostic status, and project links.
//!
//! Complies with docs/04-ux.md and NFR-S1.

use egui::{Color32, Ui};
use magicbridge_agent::DaemonStatus;

use crate::i18n::I18n;

/// Renders the About and diagnostics view.
pub fn show(ui: &mut Ui, daemon_status: Option<&DaemonStatus>, i18n: &I18n) {
    let title = i18n.t("about.title");
    let group = ui.group(|ui| {
        crate::set_accessible_name(ui, ui.id(), title);
        ui.heading(title);
        ui.label(i18n.t("about.tagline"));
        ui.add_space(6.0);

        ui.horizontal(|ui| {
            ui.label(i18n.t("about.version"));
            ui.strong(env!("CARGO_PKG_VERSION"));
        });

        ui.horizontal(|ui| {
            ui.label(i18n.t("about.github"));
            let url = i18n.t("about.github_url");
            ui.hyperlink_to(url, url);
        });

        ui.separator();

        // Daemon Diagnostic Status Card
        ui.heading(i18n.t("about.system_status"));
        ui.add_space(4.0);

        let running = daemon_status.is_some_and(|s| s.agent_running);
        ui.horizontal(|ui| {
            ui.label(i18n.t("about.agent_running"));
            if running {
                ui.colored_label(Color32::from_rgb(40, 160, 60), i18n.t("about.status_yes"));
            } else {
                ui.colored_label(Color32::from_rgb(220, 50, 40), i18n.t("about.status_no"));
            }
        });

        if let Some(status) = daemon_status {
            ui.horizontal(|ui| {
                ui.label(i18n.t("about.connected_keyboards"));
                ui.label(status.connected_keyboards.to_string());
            });

            ui.horizontal(|ui| {
                ui.label(i18n.t("about.transport"));
                let trans_str = status
                    .transport
                    .as_deref()
                    .unwrap_or_else(|| i18n.t("about.status_none"));
                ui.label(trans_str);
            });

            ui.horizontal(|ui| {
                ui.label(i18n.t("about.battery_level"));
                let batt_str = status
                    .battery_percent
                    .map(|pct| format!("{pct}%"))
                    .unwrap_or_else(|| i18n.t("about.status_none").to_string());
                ui.label(batt_str);
            });

            ui.horizontal(|ui| {
                ui.label(i18n.t("about.active_profile"));
                ui.label(&status.active_profile);
            });

            ui.horizontal(|ui| {
                ui.label(i18n.t("about.remapping_active"));
                if status.enabled && !status.paused {
                    ui.colored_label(
                        Color32::from_rgb(40, 160, 60),
                        i18n.t("about.status_active"),
                    );
                } else {
                    ui.colored_label(
                        Color32::from_rgb(220, 140, 20),
                        i18n.t("about.status_paused"),
                    );
                }
            });
        }

        ui.separator();

        if crate::is_installed_location() {
            if ui
                .button(
                    egui::RichText::new(i18n.t("about.uninstall_button"))
                        .color(Color32::from_rgb(220, 50, 40)),
                )
                .clicked()
                && let Ok(current_exe) = std::env::current_exe()
            {
                let _ = std::process::Command::new(current_exe)
                    .arg("--uninstall")
                    .spawn();
            }
            ui.add_space(4.0);
        }

        ui.label(i18n.t("about.non_admin_note"));
        ui.label(i18n.t("about.license"));
    });
    crate::set_accessible_name(ui, group.response.id, title);
}
