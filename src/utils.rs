use eframe::egui;

/// Loads a custom monospaced font that supports box-drawing characters
pub fn load_custom_font(ctx: &egui::Context) {
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
