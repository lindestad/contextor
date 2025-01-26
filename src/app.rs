use crate::formatter::{build_tree, format_file_contents, format_project_summary};
use crate::scanner::scan_project;
use arboard::Clipboard;
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
    output_expanded: bool, // Track if output is expanded or collapsed
}

impl Default for ContextorApp {
    fn default() -> Self {
        Self {
            selected_folder: None,
            output_preview: "Select a folder to generate a summary".to_string(),
            max_file_size: "1000000".to_string(), // Default 1MB
            error_message: None,
            output_expanded: false, // Start in collapsed mode
        }
    }
}

impl eframe::App for ContextorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load the custom font
        load_custom_font(ctx);

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

            // Collapsible preview window
            ui.group(|ui| {
                if self.output_expanded {
                    // Expanded View (Full Scrollable Output)
                    if ui.button("Collapse").clicked() {
                        self.output_expanded = false;
                    }

                    egui::ScrollArea::vertical()
                        .max_height(400.0) // Set max height for scrolling
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.output_preview)
                                    .font(egui::TextStyle::Monospace)
                                    .desired_rows(20),
                            );
                        });
                } else {
                    // Collapsed View (Show only first 10 lines)
                    let preview_text = self
                        .output_preview
                        .lines()
                        .take(10)
                        .collect::<Vec<_>>()
                        .join("\n");

                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(preview_text).monospace());

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("📋 Copy").clicked() {
                                copy_to_clipboard(&self.output_preview);
                            }
                            if ui.button("⬆️ Expand").clicked() {
                                self.output_expanded = true;
                            }
                        });
                    });
                }
            });
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

/// Loads a custom monospaced font that supports box-drawing characters
fn load_custom_font(ctx: &egui::Context) {
    use egui::{FontData, FontDefinitions, FontFamily};
    use std::sync::Arc;

    let mut fonts = FontDefinitions::default();

    // Load JetBrains Mono from the assets directory
    fonts.font_data.insert(
        "JetBrainsMono".to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../assets/fonts/JetBrainsMono-Regular.ttf"
        ))),
    );

    // Use JetBrains Mono for Monospace text
    fonts
        .families
        .insert(FontFamily::Monospace, vec!["JetBrainsMono".to_owned()]);

    ctx.set_fonts(fonts);
}

/// Copies text to clipboard
fn copy_to_clipboard(text: &str) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(text).unwrap();
}
