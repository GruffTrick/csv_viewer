use std::process::exit;
use csv::StringRecord;
use csv_viewer::read_from_file;


fn main() -> eframe::Result<()> {
    // let v: Vec<StringRecord> = read_from_file();

    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "CSV Viewer",
        native_options,
        Box::new(|cc| Box::new(csv_viewer::ViewerApp::new(cc))),
    )
}
