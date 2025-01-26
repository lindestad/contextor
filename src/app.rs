use crate::formatter::{build_tree, format_file_contents, format_project_summary};
use crate::scanner::scan_project;
use eframe::egui;
use rfd::FileDialog;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;

pub struct ContextorApp {
    selected_folder: Option<PathBuf>,
    output_preview: String,
    max_file_size: String, // Store as string for UI input handling
    error_message: Option<String>,
}

impl Default for ContextorApp {
    fn default() -> Self {
        Self {
            selected_folder: None,
            output_preview: "Select a folder to generate a summary".to_string(),
            max_file_size: "1000000".to_string(), // Default 1MB
            error_message: None,
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

            if let Some(err) = &self.error_message {
                ui.colored_label(egui::Color32::RED, format!("Error: {}", err));
            }

            if ui.button("Start Scan").clicked() {
                self.start_scan();
            }

            ui.separator();
            ui.label("Project Summary:");
            ui.text_edit_multiline(&mut self.output_preview);
        });
    }
}

impl ContextorApp {
    fn start_scan(&mut self) {
        self.error_message = None; // Reset errors

        let max_file_size: u64 = match self.max_file_size.parse() {
            Ok(val) if val > 0 => val,
            _ => {
                self.error_message =
                    Some("Invalid file size. Enter a positive number.".to_string());
                return;
            }
        };

        if let Some(folder) = &self.selected_folder {
            let folder_path = folder.clone();
            let (tx, rx) = mpsc::channel();

            thread::spawn(move || {
                let scan_result = scan_project(&folder_path.to_string_lossy(), max_file_size);
                tx.send(scan_result).unwrap();
            });

            match rx.recv() {
                Ok(files) => {
                    // Generate structured output
                    let tree = build_tree(&files);
                    let file_contents = format_file_contents(&files);
                    let formatted_summary = format_project_summary(tree, file_contents);

                    self.output_preview = formatted_summary;
                }
                Err(_) => self.error_message = Some("Scan failed.".to_string()),
            }
        } else {
            self.error_message = Some("No folder selected.".to_string());
        }
    }
}
