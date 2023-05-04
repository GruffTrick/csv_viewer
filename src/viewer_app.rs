#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::vec::IntoIter;
use std::fs::File;
use std::io::{BufRead, Stdin};

use csv::{Reader, ReaderBuilder, StringRecord};

use egui::style::default_text_styles;
use egui::{Align2, Context, Label, Painter, Response, Sense, Ui, Vec2};
use egui_extras::{TableBuilder, Column};
use egui_extras::{Size, StripBuilder};
use eframe::{Theme, Frame};

use rfd::FileDialog;
use atty;
use tracing_subscriber::fmt::format;

use crate::reader::*;
use crate::sort::sort_records;

#[derive(PartialEq,Debug)]
enum Delimiter { Comma, Tab, Semicolon, Auto }

struct FileInfo {
    delimiter: Delimiter, // unsure about string slice atm
    file_size_mb: f64,
    total_rows: u64,
    has_headers: bool,
}

impl Default for FileInfo {
    fn default() -> Self {
        Self {
            delimiter: Delimiter::Comma,
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

#[derive(PartialEq,Debug)]
enum DialogMessage {
    None,
    NextPage,
    PreviousPage,
    StartOfFile,
    EndOfFile,
    ExportedFile,
}

pub struct AppSettings {
    has_file: bool,
    num_rows_to_display: u64,
    current_pos: u64,
    quit_confirmation: bool,
    allowed_to_quit: bool,
    dialog_open: bool,
    dialog_msg: DialogMessage,
    index_selected_header: usize,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            has_file: false,
            num_rows_to_display: 100,
            current_pos: 0,
            quit_confirmation: false,
            allowed_to_quit: false,
            dialog_open: false,
            dialog_msg: DialogMessage::None,
            index_selected_header: 0,
        }
    }
}

pub struct ViewerApp {
    app_state: AppState,
    file_info: FileInfo,
    headers: StringRecord,
    records: Vec<StringRecord>,
    file_path: Option<String>,
    settings: AppSettings,
}

// Default values for the ViewerApp GUI
impl Default for ViewerApp {
    fn default() -> Self {
        Self {
            app_state: AppState::MainMenu,
            file_info: FileInfo::default(),
            headers: Default::default(),
            records: Vec::new(),
            file_path: None,
            settings: Default::default(),
        }
    }
}



impl eframe::App for ViewerApp {
    /// Called each time the UI needs to be repainted
    /// Widgets are placed inside of their respective panels
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {

        match self.app_state {
            AppState::MainMenu => {
                show_main_menu_window(self, ctx, frame);
            }
            AppState::Viewer => {
                show_viewer_window(self, ctx, frame);
            }
            AppState::Finder => {}
            AppState::Sorter => {
                show_sorter_window(self, ctx, frame);
            }
        }

        // If the quit confirmation setting is enabled, open the quit confirmation menu.
        if self.settings.quit_confirmation {
            show_quit_confirmation(self, ctx, frame);
        }


        // Checks if a dialog box with a required confirmation to close is set to open
        if self.settings.dialog_open == true {
            let mut dialog_msg: &str = "";
            match self.settings.dialog_msg {
                DialogMessage::None => { dialog_msg = "Error: No Dialog Message";}
                DialogMessage::NextPage => { dialog_msg = "Already on Last Page";}
                DialogMessage::PreviousPage => { dialog_msg = "Already on First Page";}
                DialogMessage::StartOfFile => {dialog_msg = "Already at Start of File";}
                DialogMessage::EndOfFile => {dialog_msg = "Already at End of File";}
                DialogMessage::ExportedFile => {dialog_msg = "Sorted File Exported Successfully"}
            }
            show_dialog_confirmation(self, ctx, dialog_msg);
        }

    }

    /// If user attempts to close the app the setting for displaying a close confirmation window
    /// is enabled.
    fn on_close_event(&mut self) -> bool {
        self.settings.quit_confirmation = true;
        self.settings.allowed_to_quit
    }

    // save current state before shutdown. EXPERIMENTAL ATM
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }
}


fn show_main_menu_window(app: &mut ViewerApp, ctx: & Context, frame: &mut eframe::Frame){
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::Window::new("Main Menu")
            .collapsible(false)
            .resizable(false)
            .anchor(Align2::CENTER_CENTER, (Vec2 { x: 0.0, y: 0.0 }))
            .show(ctx, |ui| {
                ui.heading("Settings");
                egui::widgets::global_dark_light_mode_buttons(ui);
                ui.add(
                    egui::Slider::new(&mut app.settings.num_rows_to_display, 10..=1000)
                        .logarithmic(true)
                        .text("Max Rows to Display"),
                );
                ui.label(format!("Has Headers: {:?}", app.file_info.has_headers.borrow()));
                ui.horizontal(|ui| {
                    if ui.radio_value(&mut app.file_info.has_headers,
                                      true, "Yes").clicked() {}
                    if ui.radio_value(&mut app.file_info.has_headers,
                                      false, "No").clicked() {}

                });
                ui.label(format!("Delimiter Character: {:?}", app.file_info.delimiter.borrow()));
                ui.horizontal(|ui| {
                    if ui.radio_value(&mut app.file_info.delimiter,
                                      Delimiter::Comma, "COMMA").clicked() {}
                    if ui.radio_value(&mut app.file_info.delimiter,
                                      Delimiter::Tab, "TAB").clicked() {}
                    if ui.radio_value(&mut app.file_info.delimiter,
                                      Delimiter::Semicolon, "SEMICOLON").clicked() {}
                    if ui.radio_value(&mut app.file_info.delimiter,
                                      Delimiter::Auto, "AUTO").clicked() {}
                });
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Open File").clicked() {
                        // Open From File
                        open_file(app);
                    }
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                });
                egui::warn_if_debug_build(ui);
            });
    });
}

fn show_viewer_window(app: &mut ViewerApp, ctx: & Context, frame: &mut eframe::Frame) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // top panel for a menu bar:
        egui::menu::bar(ui, |ui| {
            // Opens the file menu from the top bar
            ui.menu_button("File", |ui| {
                // Opens file dialogue window.
                if ui.button("Open").clicked() {
                    open_file(app);
                }
                // Export Changes to file
                if ui.button("(TBA)Export to...").clicked() {
                    // code here
                }
                // Closes the opened file and returns to main menu.
                if ui.button("Close").clicked() {
                    app.headers = StringRecord::new();
                    app.records = Vec::new();
                    app.file_path = None;
                    app.file_info = FileInfo::default();
                    app.settings = AppSettings::default();
                    app.app_state = AppState::MainMenu;
                }
                if ui.button("Quit").clicked() {
                    // Quit Confirmation Dialogue
                    app.settings.quit_confirmation = true;
                }
            });
            // Opens the Edit menu from the top bar
            ui.menu_button("Edit", |ui| {
                if ui.button("(TBA)Copy").clicked() {
                    // code here
                }
                if ui.button("(TBA)Paste").clicked() {
                    // code here
                }
            });
            // Opens the Data menu from the top bar
            ui.menu_button("Data", |ui| {
                if ui.button("(WIP)Sort...").clicked() {
                    app.app_state = AppState::Sorter;
                }
            });
            // Opens the Find menu from the top bar
            ui.menu_button("Navigate", |ui| {
                if ui.button("(TBA) Go To Line...").clicked() {
                    // code here
                }
                if ui.button("(TBA)Find...").clicked() {
                    // code here
                }
                if ui.button("Go To Start of File").clicked() {
                    show_first_page(app);
                }
                if ui.button("Next Page").clicked() {
                    show_next_page(app);
                }
                if ui.button("Previous Page").clicked() {
                    show_prev_page(app);
                }
                if ui.button("Go To End of File").clicked() {
                    show_last_page(app);
                }
            });
        });
    });

    // Central Panel. Displays the Cells.
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::horizontal()
            .always_show_scroll(true)
            .stick_to_bottom(true)
            .show(ui, |ui| {

            StripBuilder::new(ui)
                .size(Size::remainder().at_least(100.0)) // for the table
                .size(Size::exact(10.0)) // for the source code link
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            // Table Builder
                            build_table(app, ctx, ui);
                        });
                    });
                    // strip to separate the table from the bottom panel
                    strip.cell(|ui| {});
                });
        });
        egui::TopBottomPanel::bottom("bottom_panel").show_separator_line(true).show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                if app.file_info.has_headers {ui.label(format!("Total Rows: {}", app.file_info.total_rows.clone()));}
                else {ui.label(format!("Total Rows: {}", app.file_info.total_rows.clone()));}
                ui.label(format!("Top Pos: {}", app.settings.current_pos.clone()));
                egui::warn_if_debug_build(ui);
            });
        });
    });
}

fn build_table (app: &mut ViewerApp, ctx: & Context, ui: &mut Ui) {
    TableBuilder::new(ui).max_scroll_height(f32::INFINITY)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .striped(true) // Eventually needs to be a struct parameter
        .resizable(true) // Eventually needs to be a struct parameter
        .columns(Column::auto().resizable(true), app.headers.len())
        .column(Column::remainder())
        .header(20.0, |mut header| {
            header.col(|ui| {
                if ui.add(egui::Label::new("#").sense(Sense::click())).clicked() {
                    egui::Window::new("Clicked Row")
                        .anchor(Align2::CENTER_CENTER, (Vec2 { x: 0.0, y: 0.0 }))
                        .collapsible(false)
                        .resizable(false)
                        .show(ctx, |ui| {
                            ui.label("Clicked Header: #");
                        });
                }
            });
            for record in app.headers.iter() {
                header.col(|ui| {
                    if ui.add(egui::Label::new(format!("{}", record)).sense(Sense::click())).clicked()  {
                        egui::Window::new("Clicked header")
                            .anchor(Align2::CENTER_CENTER, (Vec2 { x: 0.0, y: 0.0 }))
                            .collapsible(false)
                            .resizable(false)
                            .show(ctx, |ui| {
                                ui.label(format!("Clicked Header: {}", record));
                            });
                    };
                });
            }
        })
        .body(|mut body| {
            for (line, record) in app.records.iter().enumerate() {
                body.row(30.0, |mut row| {
                    // display row index
                    row.col(|ui| {
                        ui.label(format!("{}", app.settings.current_pos.clone() + line as u64 + 1));
                    });
                    for column in record {
                        row.col(|ui| {
                            ui.label(format!("{}", column));
                        });
                    }
                });
            }
        });
}

fn show_next_page(app: &mut ViewerApp) {
    if app.settings.current_pos + app.settings.num_rows_to_display <= app.file_info.total_rows {
        app.records = get_records_from_pos(
            app.file_path.clone(),
            app.settings.current_pos.clone() + app.settings.num_rows_to_display,
            app.settings.num_rows_to_display.clone(), app.file_info.has_headers);
        app.settings.current_pos = app.settings.current_pos + app.settings.num_rows_to_display;
        if (app.settings.current_pos + app.settings.num_rows_to_display) > app.file_info.total_rows {

        }
    } else /* No more content in file */ {
        app.settings.dialog_msg = DialogMessage::NextPage;
        app.settings.dialog_open = true;
    }
}

fn show_prev_page(app: &mut ViewerApp) {
    // Shows previous page
    // go back NUM_ROWS_TO_DISPLAY,
    // unless ( pos - display ) < 0
    if app.settings.current_pos <= app.settings.num_rows_to_display {
        app.settings.current_pos = 0;
        app.records = get_records_from_pos(
            app.file_path.clone(),
            app.settings.current_pos.clone(),
            app.settings.num_rows_to_display.clone(), app.file_info.has_headers);
    } else {
        app.settings.current_pos = app.settings.current_pos - app.settings.num_rows_to_display;
        app.records = get_records_from_pos(
            app.file_path.clone(),
            app.settings.current_pos.clone(),
            app.settings.num_rows_to_display.clone(), app.file_info.has_headers);
    }
}

fn show_first_page(app: &mut ViewerApp) {
    if app.settings.current_pos != 0 {
        app.settings.current_pos = 0;
        app.records = get_records_from_pos(
            app.file_path.clone(),
            app.settings.current_pos.clone(),
            app.settings.num_rows_to_display.clone(),
            app.file_info.has_headers);
    } else {
        app.settings.dialog_msg = DialogMessage::StartOfFile;
        app.settings.dialog_open = true
    }
}


///Shows last page with the number of rows specified.
fn show_last_page(app: &mut ViewerApp) {
    if app.file_info.total_rows > app.settings.num_rows_to_display {
        app.settings.current_pos = app.file_info.total_rows - app.settings.num_rows_to_display;
        app.records = get_records_from_pos(
            app.file_path.clone(),
            app.settings.current_pos.clone(),
            app.settings.num_rows_to_display.clone(), app.file_info.has_headers);
    } else {
        app.settings.dialog_msg = DialogMessage::EndOfFile;
        app.settings.dialog_open = true;
    }
}


/// Open a delimited data file and read in their headers and records.
/// Uses the OS file dialog window by utilising RUSTY FILE DIALOGS by .
fn open_file(app: &mut ViewerApp) {
    if let Some(path) = FileDialog::new().pick_file() {
        app.file_path = Option::from(path.display().to_string());
        app.file_info.total_rows = get_row_count(app.file_path
            .clone());
        let mut reader = ReaderBuilder::new().has_headers(app.file_info.has_headers).from_path(app.file_path.clone().unwrap()).unwrap();
        app.headers = get_headers_from_file(reader.borrow_mut());
        app.records = get_records_from_pos(app.file_path.clone(),
                                           app.settings.current_pos.clone(),
                                           app.settings.num_rows_to_display,
                                           app.file_info.has_headers);
        app.app_state = AppState::Viewer;
    }
}


fn show_sorter_window(app: &mut ViewerApp, ctx: & Context, frame: &mut Frame) {
    let mut current_index = 0;
    let mut output_path = String::from("");

    egui::CentralPanel::default().show(ctx, |ui| {
        egui::Window::new("Sort File")
            .collapsible(false)
            .resizable(true)
            .anchor(Align2::CENTER_CENTER, (Vec2 { x: 0.0, y: 0.0 }))
            .show(ctx, |ui| {
                ui.heading("File Info");
                ui.label(format!("Filepath: {}", app.file_path.clone().unwrap()));
                ui.separator();

                ui.heading("Header Fields:");
                // ui.label(format!("{:?}", app.headers));
                ui.horizontal_wrapped(|ui| {
                    for header in app.headers.into_iter() {
                        if ui.button(header).clicked() { app.settings.index_selected_header = current_index.clone()};
                        current_index = current_index + 1;
                    }
                });
                ui.label(format!("Selected Header: {:?}", app.headers.get(app.settings.index_selected_header).unwrap()));
                ui.separator();

                ui.horizontal_wrapped(|ui| {
                    if ui.button("Sort and Export as...").clicked() {
                        ui.add(egui::widgets::Spinner::new());

                        // Choose Export path
                        if let Some(path) = FileDialog::new().save_file() {
                            output_path = path.display().to_string();
                            match sort_records(app.file_path.clone().unwrap(), output_path.clone(),
                                               app.settings.index_selected_header,) {
                                Ok(_) => {
                                    app.settings.current_pos = 0;
                                    app.file_path = Option::from(output_path.clone());
                                    app.file_info.total_rows = get_row_count(app.file_path.clone());
                                    app.records = get_records_from_pos(
                                        app.file_path.clone(),
                                        0,
                                        app.settings.num_rows_to_display,
                                        true);


                                    app.settings.dialog_msg = DialogMessage::ExportedFile;
                                    app.settings.dialog_open = true;
                                }
                                Err(_) => {println!("Error: Cannot Sort Records");}
                            }
                        }
                    }
                    if ui.button("Return to File").clicked() { app.app_state = AppState::Viewer;}
                });
                egui::warn_if_debug_build(ui);
            });
    });
}

/// Opens a dialog box within the eframe that displays passed string slice.
/// The dialog box window remains open on top of the displayed content until the "okay" button is
/// clicked by the user.
fn show_dialog_confirmation(app: &mut ViewerApp, ctx: & Context, text: &str) {
    egui::Window::new(text)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2 { x: 0.0, y: 0.0 })
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Okay").clicked() {
                    app.settings.dialog_msg = DialogMessage::None;
                    app.settings.dialog_open = false;
                }
            });
        });
}


/// Opens a dialog window that checks if the user really wants to quit the application or not.
/// "Yes!" closes the Application.
/// "Cancel" closes the confirmation window and returns to the previous window.
fn show_quit_confirmation(app: &mut ViewerApp, ctx: & Context, frame: &mut Frame) {
    egui::Window::new("Do you want to quit?")
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, Vec2 { x: 0.0, y: 0.0 })
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    app.settings.quit_confirmation = false;
                }
                if ui.button("Yes!").clicked() {
                    app.settings.allowed_to_quit = true;
                    frame.close();
                }
            });
        });
}


/// Launches the GUI for the Viewer App
pub fn run_app() -> eframe::Result<()> {
    let mut viewer_app = ViewerApp::default();

    // checks if run from terminal without a file passed in
    if atty::isnt(atty::Stream::Stdin){
        // Open from passed file
        let mut reader: Reader<Stdin> = get_reader_stdin();
        viewer_app = ViewerApp {
            app_state: AppState::Viewer,
            file_info: Default::default(),
            headers: get_headers_stdin(reader.borrow_mut()),
            records: get_records_stdin(reader.borrow_mut()),
            file_path: None,
            settings: Default::default(),
        };
    }

    let mut eframe_options = eframe::NativeOptions::default();
    eframe_options.maximized = true;

    eframe::run_native(
        "CSV Viewer",
        eframe_options,
        Box::new(|cc| Box::new(viewer_app)),
    )
}