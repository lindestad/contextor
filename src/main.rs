use contextor::app::ContextorApp;
use eframe::NativeOptions;

fn main() -> eframe::Result<()> {
    let options = NativeOptions::default();
    eframe::run_native(
        "Contextor",
        options,
        Box::new(|_cc| Ok(Box::new(ContextorApp::default()))),
    )
}
