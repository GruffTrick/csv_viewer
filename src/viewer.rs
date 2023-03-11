#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::BorrowMut;
use std::vec::IntoIter;
use csv::StringRecord;
use egui::accesskit::Size;
use egui::style::default_text_styles;
use rfd::FileDialog;
use tracing_subscriber::fmt::format;

use crate::reader::{get_reader_file, get_headers_file, get_records_file};


pub const MAX_NUM_ROWS: i32 = 1000;
pub const MAX_NUM_COLUMNS: i32 = 1000;


pub struct ViewerApp {
    headers: StringRecord,
    records: Vec<StringRecord>,
    file_path: Option<String>,
}

impl Default for ViewerApp {
    fn default() -> Self {
        Self {
            headers: Default::default(),
            records: Vec::new(),
            file_path: None,
        }
    }
}

impl ViewerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for ViewerApp {

    /// Called each time the UI needs to be repainted
    /// Widgets are placed inside of their respective panels
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        // initiate self
        // let Self { headers, records, file_path } = self;

        //#[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // top panel for a menu bar:
            egui::menu::bar(ui, |ui| {
                // Opens the file menu from the top bar
                ui.menu_button("File", |ui| {
                    // Opens file dialogue window.
                    if ui.button("Open").clicked() {
                        if let Some(path) = FileDialog::new().pick_file() {
                            self.file_path = Some(path.display().to_string());
                            let mut reader = get_reader_file(self.file_path.clone());
                            self.headers = get_headers_file(reader.borrow_mut());
                            self.records = get_records_file(reader.borrow_mut());
                        }
                    }
                    // Export Changes to file
                    if ui.button("Export as...").clicked() {

                        // code here

                    }
                    // Closes the frame and ends the application.
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        // Central Panel. Displays the Cells.
        egui::CentralPanel::default().show(ctx, |ui| {

            egui::ScrollArea::both().show(ui, |ui| {
                egui::Grid::new("some_unique_id").show(ui, |ui| {

                    // display headers
                    for record in self.headers.iter() {
                        ui.label(format!("{}", record));
                    }
                    ui.end_row();

                    // display records
                    for (row, record) in self.records.iter().enumerate() {
                        for column in record {
                            ui.label(format!("{}", column));
                        }
                        ui.end_row();
                    }
                });
            });
            egui::warn_if_debug_build(ui);
        });

        // Bottom panel for displaying contextual info like the debug identifier and coordinates.
        // CURRENTLY OBFUSCATES THE BOTTOM SCROLL BAR!!
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::warn_if_debug_build(ui);
        });
    }

    // Called by the frame work to save current state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }
}


pub fn run_app(headers: StringRecord, records: Vec<StringRecord>, file_path: Option<String>) -> eframe::Result<()> {

    let mut v = ViewerApp {
        headers,
        records,
        file_path: None,
    };

    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "CSV Viewer",
        native_options,
        Box::new(|cc| Box::new(v)),
    )
}