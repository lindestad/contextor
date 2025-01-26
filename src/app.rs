use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;

pub struct ContextorApp {
    selected_folder: Option<PathBuf>,
    output_preview: String,
    max_file_size: String, // Store as string for UI input handling
}

impl Default for ContextorApp {
    fn default() -> Self {
        Self {
            selected_folder: None,
            output_preview: "Select a folder to generate a summary".to_string(),
            max_file_size: "1000000".to_string(), // Default 1MB
        }
    }
}

impl eframe::App for ContextorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Contextor");

            if ui.button("Select Folder").clicked() {
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.selected_folder = Some(path.clone());
                    self.output_preview = format!("Selected folder: {:?}", path);
                }
            }

            ui.separator();
            ui.label("Max file size to scan (in bytes):");
            ui.text_edit_singleline(&mut self.max_file_size);

            ui.separator();
            ui.label("Project Summary:");
            ui.text_edit_multiline(&mut self.output_preview);
        });
    }
}
