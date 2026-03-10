use eframe::egui;

use crate::installer::Installer;
use crate::package::PackageState;

pub struct GUIFrontend {
    installer: Installer,
}

impl GUIFrontend {
    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self { installer: Installer::new() }
    }
}

impl eframe::App for GUIFrontend {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        let had_events = self.installer.poll_events();
        if had_events {
            context.request_repaint();
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default().inner_margin(15))
            .show(context, |ui| {
                ui.heading("KeTCindy Installer");
                let packages = self.installer.packages();

                for package in packages {
                    ui.indent(format!("package_{}", package.kind), |ui| {
                        ui.label(format!("{}", package.kind));
                        ui.horizontal(|ui| {
                            match package.state {
                                PackageState::Initialized => {
                                    ui.label("Initialized");
                                    self.installer.fetch_versions(package.kind);
                                },
                                PackageState::Fetching => {
                                    ui.spinner();
                                    ui.label("Fetching...");
                                },
                                PackageState::Fetched { versions, selected_index } => {
                                    let mut is_selected = selected_index.is_some();
                                    if ui.checkbox(&mut is_selected, "").changed() {
                                        if is_selected {
                                            self.installer.select_version(package.kind, Some(0));
                                        } else {
                                            self.installer.select_version(package.kind, None);
                                        }
                                    }
                                    ui.add_enabled_ui(is_selected, |ui| {
                                        if let Some(index) = selected_index {
                                            let mut temp_index = index;
                                            if egui::ComboBox::from_id_salt(format!("versions_{}", package.kind))
                                                .show_index(ui, &mut temp_index, versions.len(), |i| &versions[i])
                                                .changed()
                                            {
                                                self.installer.select_version(package.kind, Some(temp_index));
                                            }
                                        } else {
                                            ui.label("(will not be installed)");
                                        }
                                    });
                                },
                                PackageState::Downloading { progress } => {
                                    ui.add(egui::ProgressBar::new(progress).show_percentage());
                                },
                                PackageState::Downloaded { .. } => {
                                    ui.label("downloaded");
                                },
                                PackageState::Installing => {
                                    ui.spinner();
                                    ui.label("Installing...");
                                },
                                PackageState::Installed => {
                                    ui.label("Installed");
                                },
                                PackageState::Error(msg) => {
                                    ui.colored_label(egui::Color32::RED, format!("Error: {}", msg));
                                }
                            }
                        });
                    });
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                    let can_install = self.installer.packages().iter().any(|p| {
                        matches!(p.state, PackageState::Fetched { selected_index: Some(_), .. })
                    });
                    if ui.add_enabled(can_install, egui::Button::new("Install")).clicked() {
                        self.installer.start_installation();
                    }
                });
            });
    }
}
