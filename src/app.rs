use std::sync::Arc;

use eframe::egui;

use crate::package::Package;
use crate::package::PackageKind;

pub struct GUIFrontend {
    ketcindy: Arc<Package>,
    cinderella: Arc<Package>,
    r: Arc<Package>,
    maxima: Arc<Package>,
    packages_fetched: bool,
    async_runtime: tokio::runtime::Runtime,
    current_page_index: usize,
}

enum Page {
    SelectVersion,
}

impl GUIFrontend {
    const PAGES: &[Page] = &[
        Page::SelectVersion,
    ];

    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self {
            ketcindy: Arc::new(Package::new(PackageKind::KeTCindy)),
            cinderella: Arc::new(Package::new(PackageKind::Cinderella)),
            r: Arc::new(Package::new(PackageKind::R)),
            maxima: Arc::new(Package::new(PackageKind::Maxima)),
            packages_fetched: false,
            async_runtime: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap(),
            current_page_index: 0,
        }
    }

    fn select_version(&mut self, ui: &mut egui::Ui) {
        if self.packages_fetched == false {
            self.packages_fetched = true;

            let packages = vec![
                self.ketcindy.clone(),
                self.cinderella.clone(),
                self.r.clone(),
                self.maxima.clone(),
            ];
            for package in packages {
                self.async_runtime.spawn(async move {
                    if let Err(e) = package.fetch_versions().await {
                        eprintln!("{e}");
                    }
                });
            }
        }

        ui.heading("1. Select package version");

        for (name, package) in [
            ("KeTCindy", &self.ketcindy),
            ("Cinderella", &self.cinderella),
            ("R", &self.r),
            ("Maxima", &self.maxima),
        ] {
            ui.indent(format!("indent_{name}"), |ui| {
                ui.label(format!("{name}:"));
                if let Some(versions) = &*package.versions.lock().unwrap() {
                    egui::ComboBox::from_id_salt(format!("version_select_{name}")).show_index(
                        ui,
                        &mut *package.selected_version_index.lock().unwrap(),
                        versions.len(),
                        |i| versions[i].clone(),
                    );
                } else {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label("loading...");
                    });
                }
            });
        }
    }
}

impl eframe::App for GUIFrontend {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().inner_margin(15))
            .show(context, |ui| {
                match Self::PAGES[self.current_page_index] {
                    Page::SelectVersion => self.select_version(ui),
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::BOTTOM), |ui| {
                    ui.horizontal(|ui| {
                        // disable the `Next >` button on the last page
                        ui.add_enabled_ui(
                            self.current_page_index + 1 < Self::PAGES.len(),
                            |ui| {
                                if ui.button("Next >").clicked() {
                                    self.current_page_index += 1;
                                }
                            }
                        );
                        // disable the `< Back` button on the first page
                        ui.add_enabled_ui(
                            0 < self.current_page_index,
                            |ui| {
                                if ui.button("< Back").clicked() {
                                    self.current_page_index -= 1;
                                }
                            }
                        );
                    });
                });
            });
    }
}
