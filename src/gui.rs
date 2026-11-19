use crate::zuza;
use eframe::egui;
use rfd;
use std::path::PathBuf;

pub struct App {
    path: Option<PathBuf>,
    text: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            path: None,
            text: String::from("Keine Datei ausgewählt"),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Zusatztexte ergänzen");

                ui.add_space(5.0);

                // CSV auswählen
                if ui.button("CSV öffnen").clicked() {
                    self.path = rfd::FileDialog::new()
                        .add_filter("CSV", &["csv"])
                        .pick_file();
                }

                // Ausgewählte Datei anzeigen
                if let Some(path) = &self.path {
                    let filename = path
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("Unbekannte Datei");

                    ui.label(filename);
                } else {
                    ui.label(&self.text);
                }

                ui.add_space(5.0);

                // Button nur aktivieren, wenn eine Datei ausgewählt wurde
                let enabled = self.path.is_some();
                if ui
                    .add_enabled(enabled, egui::Button::new("Texte ergänzen"))
                    .clicked()
                {
                    if let Some(path) = &self.path {
                        let Some(path_string) = path.to_str() else {
                            self.text = String::from("FEHLER: Wähle eine gültige Datei aus");
                            return;
                        };

                        let text = zuza::read_input(path_string);

                        let token = zuza::complement(text);

                        if let Err(_) = zuza::write_output(token, path_string) {
                            self.text = String::from("FEHLER: Umwandlung fehlgeschlagen.");
                        } else {
                            self.text = String::from("Umwandlung erfolgreich.");

                            self.path = None;
                        }
                    }
                }
            });
        });
    }
}

pub fn run() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 200.0])
            .with_min_inner_size([500.0, 200.0])
            .with_max_inner_size([500.0, 200.0]),
        ..Default::default()
    };

    eframe::run_native(
        "ZuZa",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}
