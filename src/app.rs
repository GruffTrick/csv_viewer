#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::BorrowMut;
use std::vec::IntoIter;
use csv::{Reader, StringRecord};
use egui::accesskit::Size;
use egui::style::default_text_styles;
use rfd::FileDialog;
use tracing_subscriber::fmt::format;
use egui_extras::{TableBuilder, Column};
use std::fs::File;
use std::io::{BufRead, Stdin};
use atty;
use egui::{Context, Painter};

use crate::reader::*;


pub const MAX_NUM_ROWS: i32 = 1000;
pub const MAX_NUM_COLUMNS: i32 = 1000;

// pub struct MainMenu {
//     // Parameters here.
// }
//
// impl eframe::App for MainMenu {
//     fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui|
//             {
//                 if ui.button("Open File").clicked() {
//                     // self::egui_glow::Painter::destroy(self::egui_glow::Painter);
//                     frame.close();
//                 }
//                 if ui.button("Quit").clicked() {
//                     frame.close();
//                 }
//             }
//         );
//     }
// }

enum AppType {
    MainMenu,
    Viewer,
    // Finder,
    // Sorter,
}

pub struct ViewerApp {
    app: AppType,
    headers: StringRecord,
    records: Vec<StringRecord>,
    file_path: Option<String>,
}

// Default values for the ViewerApp GUI
impl Default for ViewerApp {
    fn default() -> Self {
        Self {
            app: AppType::MainMenu,
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

        match self.app {
            AppType::MainMenu => {
                egui::CentralPanel::default().show(ctx, |ui| {
                if ui.button("Open File").clicked() {
                    // Open From File
                    if let Some(path) = FileDialog::new().pick_file() {
                        let file_path = Some(path.display().to_string());
                        let mut reader: Reader<File> = get_reader_file(file_path.clone());
                        self.headers = get_headers_file(reader.borrow_mut());
                        self.records = get_records_file(reader.borrow_mut());
                        self.app = AppType::Viewer;
                    }
                }
                if ui.button("Quit").clicked() {
                    _frame.close();
                }
            }
        );

            }
            AppType::Viewer => {
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
                            if ui.button("(WIP)Export as...").clicked() {
                                // code here
                            }
                            // Closes the frame and ends the application.
                            if ui.button("Quit").clicked() {
                                _frame.close();
                            }
                        });

                        ui.menu_button("Edit", |ui| {
                            if ui.button("(WIP)").clicked() {
                                // code here
                            }
                        });

                        ui.menu_button("Data", |ui| {
                            if ui.button("Sort").clicked() {
                                // code here
                            }
                        });

                        ui.menu_button("Find", |ui| {
                            if ui.button("Find Cell").clicked() {
                                // code here
                            }
                        });
                    });
                });

                // Central Panel. Displays the Cells.
                egui::CentralPanel::default().show(ctx, |ui| {
                    // Table Builder
                    TableBuilder::new(ui)
                        .striped(true) // Eventually needs to be a struct parameter
                        .resizable(true) // Eventually needs to be a struct parameter
                        .columns(Column::auto().resizable(true), self.headers.len() - 1)
                        .column(Column::remainder())
                        .header(20.0, |mut header| {
                            for record in self.headers.iter() {
                                header.col(|ui| {
                                    ui.heading(format!("{}", record));
                                });
                            }
                        })
                        .body(|mut body| {
                            for (line, record) in self.records.iter().enumerate() {
                                body.row(30.0, |mut row| {
                                    for column in record {
                                        row.col(|ui| {
                                            ui.label(format!("{}", column));
                                        });
                                    }
                                });
                            }
                        });
                    // end of table
                });
            }
        }
        // Bottom panel for displaying contextual info like the debug identifier and coordinates.
        // CURRENTLY OBFUSCATES THE BOTTOM SCROLL BAR!!
        // egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        //     egui::warn_if_debug_build(ui);
        // });
    }

    // Called by the frame work to save current state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }
}


/// Launches the GUI for the Viewer App
pub fn run_app() -> eframe::Result<()> {

    let mut viewer_app = ViewerApp::default();

    // checks if run from terminal without a file passed in
    if atty::isnt(atty::Stream::Stdin){
        // Open from passed file
        println!("Reading from Terminal Stdin");
        let mut reader: Reader<Stdin> = get_reader_stdin();
        // let headers = get_headers_stdin(reader.borrow_mut());
        // let records = get_records_stdin(reader.borrow_mut());
        viewer_app = ViewerApp {
            app: AppType::Viewer,
            headers: get_headers_stdin(reader.borrow_mut()),
            records: get_records_stdin(reader.borrow_mut()),
            file_path: None
        };
    }else {
        // Open from file
        // viewer_app = ViewerApp {
        //     app: AppType::MainMenu,
        //     headers,
        //     records,
        //     file_path: None
        // };
        //// Open From File
        // if let Some(path) = FileDialog::new().pick_file() {
        //     let file_path = Some(path.display().to_string());
        //     let mut reader: Reader<File> = get_reader_file(file_path.clone());
        //     let headers = get_headers_file(reader.borrow_mut());
        //     let records = get_records_file(reader.borrow_mut());
        // }
    }

    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "CSV Viewer",
        native_options,
        Box::new(|cc| Box::new(viewer_app)),
    )
}