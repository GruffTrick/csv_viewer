#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod reader;

use std::borrow::BorrowMut;
use std::vec::IntoIter;
use std::fs::File;
use std::io::{BufRead, Stdin};

use csv::{Reader, ReaderBuilder, StringRecord};

use egui::accesskit::Size;
use egui::style::default_text_styles;
use egui::{Align2, Context, Painter, Vec2};
use egui_extras::{TableBuilder, Column};

use rfd::FileDialog;
use atty;


use crate::reader::*;

struct FileInfo {
    delimiter_char: char, // unsure about string slice atm
    file_size_mb: f64,
    total_rows: usize,
    has_headers: bool,
}

impl Default for FileInfo {
    fn default() -> Self {
        Self {
            delimiter_char: ',',
            file_size_mb: 0.0,
            total_rows: 0,
            has_headers: true,
        }

    }
}


enum AppState {
    MainMenu,
    Viewer,
    Finder,
    Sorter,
}

pub struct AppSettings {
    has_file: bool,
    num_rows_to_display: u64,
    row_pos: u64,
    quit_confirmation: bool,
    allowed_to_quit: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            has_file: false,
            num_rows_to_display: 3,
            row_pos: 4,
            quit_confirmation: false,
            allowed_to_quit: false,
        }
    }
}

pub struct ViewerApp {
    app: AppState,
    file_info: FileInfo,
    headers: StringRecord,
    records: Vec<StringRecord>,
    row_count: u64,
    file_path: Option<String>,
    settings: AppSettings,
}

// Default values for the ViewerApp GUI
impl Default for ViewerApp {
    fn default() -> Self {
        Self {
            app: AppState::MainMenu,
            file_info: FileInfo::default(),
            headers: Default::default(),
            records: Vec::new(),
            row_count: 0,
            file_path: None,
            settings: Default::default(),
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        match self.app {
            AppState::MainMenu => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Open File").clicked() {
                            // Open From File
                            if let Some(path) = FileDialog::new().pick_file() {
                                self.file_path = Option::from(path.display().to_string());
                                let file_path = Some(path.display().to_string());
                                let mut reader: Reader<File> = get_reader_from_file(file_path.clone());
                                self.headers = get_headers_from_file(reader.borrow_mut());
                                self.records = get_records_file(reader.borrow_mut());
                                self.app = AppState::Viewer;
                            }
                        }
                        if ui.button("Quit").clicked() {
                            frame.close();
                        }
                    });
                    egui::warn_if_debug_build(ui);
                });
            }
            AppState::Viewer => {
                egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                    // top panel for a menu bar:
                    egui::menu::bar(ui, |ui| {
                        // Opens the file menu from the top bar
                        ui.menu_button("File", |ui| {
                            // Opens file dialogue window.
                            if ui.button("Open").clicked() {
                                if let Some(path) = FileDialog::new().pick_file() {
                                    self.file_path = Some(path.display().to_string());
                                    let mut reader = get_reader_from_file(self.file_path.clone());
                                    self.headers = get_headers_from_file(reader.borrow_mut());
                                    self.records = get_records_file(reader.borrow_mut());
                                }
                            }
                            // Export Changes to file
                            if ui.button("(WIP)Export to...").clicked() {
                                // code here
                            }
                            // Closes the frame and ends the application.
                            if ui.button("Close").clicked() {
                                self.app = AppState::MainMenu;
                            }
                            if ui.button("Quit").clicked() {
                                // Quit Confirmation Dialogue
                                self.settings.quit_confirmation = true;

                                }
                        });
                        // Opens the Edit menu from the top bar
                        ui.menu_button("Edit", |ui| {
                            if ui.button("(WIP)Copy").clicked() {
                                // code here
                            }
                            if ui.button("(WIP)Paste").clicked() {
                                // code here
                            }
                        });
                        ui.menu_button("View", |ui| {
                            if ui.button("(WIP) Go To Line...").clicked() {
                                // code here
                            }
                            if ui.button("(WIP) Go To First Page").clicked() {
                                // code here
                            }
                            if ui.button("(WIP) Go To Last Page").clicked() {
                                // code here
                            }
                        });
                        // Opens the Data menu from the top bar
                        ui.menu_button("Data", |ui| {
                            if ui.button("(WIP )Sort...").clicked() {
                                // code here
                            }
                        });
                        // Opens the Find menu from the top bar
                        ui.menu_button("Navigate", |ui| {
                            if ui.button("(WIP)Search file...").clicked() {
                                // code here
                            }
                            if ui.button("Next Page").clicked() {
                                self.records = get_records_from_pos(self.file_path.clone(),
                                                                    self.settings.row_pos,
                                                                    self.settings.num_rows_to_display)
                            }
                            if ui.button("Previous Page").clicked() {

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
                        .columns(Column::auto().resizable(true), self.headers.len())
                        .column(Column::remainder())
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading(format!(" "));
                            });
                            for record in self.headers.iter() {
                                header.col(|ui| {
                                    ui.heading(format!("{}", record));
                                });
                            }
                        })
                        .body(|mut body| {
                            for (line, record) in self.records.iter().enumerate() {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(format!("{}", line));
                                    });
                                    for column in record {
                                        row.col(|ui| {
                                            ui.label(format!("{}", column));
                                        });
                                    }
                                });
                            }
                        });
                });
            }
            AppState::Finder => {}
            AppState::Sorter => {}
        }

        if self.settings.quit_confirmation {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .anchor(Align2::CENTER_CENTER, (Vec2 { x: 0.0, y: 0.0 }))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("Cancel").clicked() {
                            self.settings.quit_confirmation = false;
                        }

                        if ui.button("Yes!").clicked() {
                            self.settings.allowed_to_quit = true;
                            frame.close();
                        }
                    });
                });
        }

        // Bottom panel for displaying contextual info like the debug identifier and coordinates.
        // CURRENTLY OBFUSCATES THE BOTTOM SCROLL BAR!!
        // egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        //     egui::warn_if_debug_build(ui);
        // });
    }

    fn on_close_event(&mut self) -> bool {
        self.settings.quit_confirmation = true;
        self.settings.allowed_to_quit
    }

    // save current state before shutdown. EXPERIMENTAL ATM
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
            app: AppState::Viewer,
            file_info: Default::default(),
            headers: get_headers_stdin(reader.borrow_mut()),
            records: get_records_stdin(reader.borrow_mut()),
            row_count: 0,
            file_path: None,
            settings: Default::default(),
        };
        let mut reader = Reader::from_path("uspop.csv");
    }

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "CSV Viewer",
        native_options,
        Box::new(|cc| Box::new(viewer_app)),
    )
}


fn main() {

    run_app().expect("Runtime Error");

}
