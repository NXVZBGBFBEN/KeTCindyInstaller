use eframe::egui;
use std::sync::Arc;
use std::sync::Mutex;

pub struct GUIFrontend {
    ketcindy_version_fetched: bool,
    ketcindy_selected_release_index: usize,
    ketcindy_releases: Arc<Mutex<Option<Vec<octocrab::models::repos::Release>>>>,
    cinderella_version_fetched: bool,
    cinderella_release: Arc<Mutex<Option<crate::utils::HomebrewResponse>>>,
    async_runtime: tokio::runtime::Runtime,
    current_page_index: usize,
}

enum Page {
    InstallKeTCindy,
    InstallCinderella,
    InstallR,
    InstallMaxima,
}

impl GUIFrontend {
    const PAGES: &[Page] = &[
        Page::InstallKeTCindy,
        Page::InstallCinderella,
        Page::InstallR,
        Page::InstallMaxima,
    ];

    pub fn new(_creation_context: &eframe::CreationContext<'_>) -> Self {
        Self {
            ketcindy_version_fetched: false,
            ketcindy_selected_release_index: 0,
            ketcindy_releases: Arc::new(Mutex::new(None)),
            cinderella_version_fetched: false,
            cinderella_release: Arc::new(Mutex::new(None)),
            async_runtime: tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .unwrap(),
            current_page_index: 0,
        }
    }

    fn install_ketcindy(&mut self, ui: &mut egui::Ui) {
        if self.ketcindy_version_fetched == false {
            let ketcindy_releases = self.ketcindy_releases.clone();
            self.async_runtime.spawn(async move {
                *ketcindy_releases.lock().unwrap() =
                    Some(crate::utils::fetch_github_releases("ketpic", "ketcindy").await);
            });
            self.ketcindy_version_fetched = true;
        }
        ui.heading("1. Installing KeTCindy");
        ui.indent("indent_test", |ui| {
            ui.label("select version:");
            if let Some(ketcindy_releases) = &*self.ketcindy_releases.lock().unwrap() {
                egui::ComboBox::from_id_salt("version_select").show_index(
                    ui,
                    &mut self.ketcindy_selected_release_index,
                    ketcindy_releases.len(),
                    |i| ketcindy_releases[i].tag_name.clone(),
                );
            } else {
                ui.horizontal(|ui| {
                    ui.spinner();
                    ui.label("loading...");
                });
            }
        });
    }

    fn install_cinderella(&mut self, ui: &mut egui::Ui) {
        if self.cinderella_version_fetched == false {
            let cinderella_release = self.cinderella_release.clone();
            self.async_runtime.spawn(async move {
                *cinderella_release.lock().unwrap() =
                    Some(crate::utils::fetch_homebrew_latest_release("cinderella").await);
            });
            self.cinderella_version_fetched = true;
        }
        ui.heading("2. Installing Cinderella");
        ui.indent("indent", |ui| {
            ui.label("test");
            if let Some(cinderella_release) = &*self.cinderella_release.lock().unwrap() {
                ui.label(format!("{}", cinderella_release.version));
            };
        });
    }
}

impl eframe::App for GUIFrontend {
    fn update(&mut self, context: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().inner_margin(15))
            .show(context, |ui| {
                match Self::PAGES[self.current_page_index] {
                    Page::InstallKeTCindy => self.install_ketcindy(ui),
                    Page::InstallCinderella => self.install_cinderella(ui),
                    Page::InstallR => (),
                    Page::InstallMaxima => (),
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
