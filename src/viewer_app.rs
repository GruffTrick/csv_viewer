#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::{Borrow, BorrowMut};
use std::vec::IntoIter;
use std::fs::File;
use std::io::{BufRead, Stdin};

use csv::{Reader, ReaderBuilder, StringRecord};

use egui::accesskit::Size;
use egui::style::default_text_styles;
use egui::{Align2, Context, Label, Painter, Response, Sense, Ui, Vec2};
use egui_extras::{TableBuilder, Column};

use rfd::FileDialog;
use atty;
use eframe::{Theme, Frame};


use crate::reader::*;

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
}

pub struct AppSettings {
    has_file: bool,
    num_rows_to_display: u64,
    current_pos: u64,
    quit_confirmation: bool,
    allowed_to_quit: bool,
    dialog_open: bool,
    dialog_msg: DialogMessage,
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
        }
    }
}

pub struct ViewerApp {
    app: AppState,
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
            app: AppState::MainMenu,
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

        match self.app {
            AppState::MainMenu => {
                show_main_menu_window(self, ctx, frame);
            }
            AppState::Viewer => {
                show_viewer_window(self, ctx, frame);
                // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                //     // top panel for a menu bar:
                //     egui::menu::bar(ui, |ui| {
                //         // Opens the file menu from the top bar
                //         ui.menu_button("File", |ui| {
                //             // Opens file dialogue window.
                //             if ui.button("Open").clicked() {
                //                 if let Some(path) = FileDialog::new().pick_file() {
                //                     self.settings.current_pos = 0;
                //                     self.file_path = Option::from(path.display().to_string());
                //                     self.file_info.total_rows = get_row_count(self.file_path
                //                         .clone());
                //                     let mut reader: Reader<File> = get_reader_from_file(self.file_path.clone());
                //                     self.headers = get_headers_from_file(reader.borrow_mut());
                //                     // self.records = get_records_file(reader.borrow_mut());
                //                     self.records = get_records_from_pos(self.file_path.clone(), self.settings.current_pos.clone(), self.settings.num_rows_to_display,self.file_info.has_headers);
                //                     self.app = AppState::Viewer;
                //
                //                 }
                //             }
                //             // Export Changes to file
                //             if ui.button("(TBA)Export to...").clicked() {
                //                 // code here
                //             }
                //             // Closes the frame and ends the application.
                //             if ui.button("Close").clicked() {
                //                 self.headers = StringRecord::new();
                //                 self.records = Vec::new();
                //                 self.file_path = None;
                //                 self.file_info = FileInfo::default();
                //                 self.settings = AppSettings::default();
                //                 self.app = AppState::MainMenu;
                //             }
                //             if ui.button("Quit").clicked() {
                //                 // Quit Confirmation Dialogue
                //                 self.settings.quit_confirmation = true;
                //             }
                //         });
                //         // Opens the Edit menu from the top bar
                //         ui.menu_button("Edit", |ui| {
                //             if ui.button("(TBA)Copy").clicked() {
                //                 // code here
                //             }
                //             if ui.button("(TBA)Paste").clicked() {
                //                 // code here
                //             }
                //         });
                //         // Opens the Data menu from the top bar
                //         ui.menu_button("Data", |ui| {
                //             if ui.button("(TBA)Sort...").clicked() {
                //                 // code here
                //             }
                //         });
                //         // Opens the Find menu from the top bar
                //         ui.menu_button("Navigate", |ui| {
                //             if ui.button("(TBA) Go To Line...").clicked() {
                //                 // code here
                //             }
                //             if ui.button("(TBA)Find...").clicked() {
                //                 // code here
                //             }
                //             if ui.button("Next Page").clicked() {
                //                 if self.settings.current_pos + self.settings.num_rows_to_display <= self.file_info.total_rows {
                //                     self.records = get_records_from_pos(
                //                         self.file_path.clone(),
                //                         self.settings.current_pos.clone() + self.settings.num_rows_to_display,
                //                         self.settings.num_rows_to_display.clone(), self.file_info.has_headers);
                //                     self.settings.current_pos = self.settings.current_pos + self.settings.num_rows_to_display;
                //                     if (self.settings.current_pos + self.settings.num_rows_to_display) > self.file_info.total_rows {
                //
                //                     }
                //                 } else /* No more content in file */ {
                //                     self.settings.dialog_msg = DialogMessage::NextPage;
                //                     self.settings.dialog_open = true;
                //                 }
                //             }
                //             if ui.button("Previous Page").clicked() {
                //                 // Shows previous page
                //                 // go back NUM_ROWS_TO_DISPLAY,
                //                 // unless ( pos - display ) < 0
                //                 if self.settings.current_pos <= self.settings.num_rows_to_display {
                //                     self.settings.current_pos = 0;
                //                     self.records = get_records_from_pos(
                //                         self.file_path.clone(),
                //                         self.settings.current_pos.clone(),
                //                         self.settings.num_rows_to_display.clone(), self.file_info.has_headers);
                //                 } else {
                //                     self.settings.current_pos = self.settings.current_pos - self.settings.num_rows_to_display;
                //                     self.records = get_records_from_pos(
                //                         self.file_path.clone(),
                //                         self.settings.current_pos.clone(),
                //                         self.settings.num_rows_to_display.clone(), self.file_info.has_headers);
                //                 }
                //             }
                //             if ui.button("Go To Start of File").clicked() {
                //                 self.settings.current_pos = 0;
                //                 self.records = get_records_from_pos(
                //                     self.file_path.clone(),
                //                     self.settings.current_pos.clone(),
                //                     self.settings.num_rows_to_display.clone(), self.file_info.has_headers);
                //             }
                //             if ui.button("Go To End of File").clicked() {
                //                 if self.file_info.total_rows > self.settings.num_rows_to_display {
                //                     self.settings.current_pos = self.file_info.total_rows - self.settings.num_rows_to_display;
                //                     self.records = get_records_from_pos(
                //                         self.file_path.clone(),
                //                         self.settings.current_pos.clone(),
                //                         self.settings.num_rows_to_display.clone(), self.file_info.has_headers);
                //                 } else {
                //                     self.settings.dialog_open = true;
                //                 }
                //             }
                //         });
                //     });
                // });
                //
                // // Central Panel. Displays the Cells.
                // egui::CentralPanel::default().show(ctx, |ui| {
                //     egui::ScrollArea::horizontal().always_show_scroll(true).stick_to_bottom(true).show(ui, |ui| {
                //         use egui_extras::{Size, StripBuilder};
                //         StripBuilder::new(ui)
                //             .size(Size::remainder().at_least(100.0)) // for the table
                //             .size(Size::exact(10.0)) // for the source code link
                //             .vertical(|mut strip| {
                //                 strip.cell(|ui| {
                //                     egui::ScrollArea::horizontal().show(ui, |ui| {
                //                         // Table Builder
                //                         TableBuilder::new(ui).max_scroll_height(f32::INFINITY)
                //                             .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                //                             .striped(true) // Eventually needs to be a struct parameter
                //                             .resizable(true) // Eventually needs to be a struct parameter
                //                             .columns(Column::auto().resizable(true), self.headers.len())
                //                             .column(Column::remainder())
                //                             .header(20.0, |mut header| {
                //                                 header.col(|ui| {
                //                                     if ui.add(egui::Label::new("#").sense(Sense::click())).clicked() {
                //                                         egui::Window::new("Clicked Row")
                //                                             .anchor(Align2::CENTER_CENTER, (Vec2 { x: 0.0, y: 0.0 }))
                //                                             .collapsible(false)
                //                                             .resizable(false)
                //                                             .show(ctx, |ui| {
                //                                                 ui.label("Clicked Header: #");
                //                                             });
                //                                     }
                //                                 });
                //                                 for record in self.headers.iter() {
                //                                     header.col(|ui| {
                //                                         if ui.add(egui::Label::new(format!("{}", record)).sense(Sense::click())).clicked()  {
                //                                             egui::Window::new("Clicked header")
                //                                                 .anchor(Align2::CENTER_CENTER, (Vec2 { x: 0.0, y: 0.0 }))
                //                                                 .collapsible(false)
                //                                                 .resizable(false)
                //                                                 .show(ctx, |ui| {
                //                                                     ui.label(format!("Clicked Header: {}", record));
                //                                                 });
                //                                         };
                //                                     });
                //                                 }
                //                             })
                //                             .body(|mut body| {
                //                                 for (line, record) in self.records.iter().enumerate() {
                //                                     body.row(30.0, |mut row| {
                //                                         // display row index
                //                                         row.col(|ui| {
                //                                             ui.label(format!("{}", self.settings.current_pos.clone() + line as u64 + 1));
                //                                         });
                //                                         for column in record {
                //                                             row.col(|ui| {
                //                                                 ui.label(format!("{}", column));
                //                                             });
                //                                         }
                //                                     });
                //                                 }
                //                             });
                //                     });
                //                 });
                //                 // strip to separate the table from the bottom panel
                //                 strip.cell(|ui| {});
                //             });
                //     });
                //     egui::TopBottomPanel::bottom("bottom_panel").show_separator_line(true).show(ctx, |ui| {
                //         ui.horizontal_centered(|ui| {
                //             if self.file_info.has_headers {ui.label(format!("Total Rows: {}", self.file_info.total_rows.clone()));}
                //             else {ui.label(format!("Total Rows: {}", self.file_info.total_rows.clone()));}
                //             ui.label(format!("Top Pos: {}", self.settings.current_pos.clone()));
                //             egui::warn_if_debug_build(ui);
                //         });
                //     });
                // });
            }
            AppState::Finder => {}
            AppState::Sorter => {}
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
                                      Delimiter::Comma, "COMMA").clicked() {
                        // self.file_info.delimiter = Delimiter::Comma;
                    }
                    if ui.radio_value(&mut app.file_info.delimiter,
                                      Delimiter::Tab, "TAB").clicked() {
                        // self.file_info.delimiter = Delimiter::Tab;
                    }
                    if ui.radio_value(&mut app.file_info.delimiter,
                                      Delimiter::Semicolon, "SEMICOLON").clicked() {
                        // self.file_info.delimiter = Delimiter::Semicolon;
                    }
                    if ui.radio_value(&mut app.file_info.delimiter,
                                      Delimiter::Auto, "AUTO").clicked() {
                        // self.file_info.delimiter = Delimiter::Auto;
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Open File").clicked() {
                        // Open From File
                        if let Some(path) = FileDialog::new().pick_file() {
                            app.file_path = Option::from(path.display().to_string());
                            app.file_info.total_rows = get_row_count(app.file_path
                                .clone());
                            let mut reader: Reader<File> = get_reader_from_file(app.file_path.clone());
                            app.headers = get_headers_from_file(reader.borrow_mut());
                            app.records = get_records_from_pos(app.file_path.clone(),
                                                               app.settings.current_pos.clone(),
                                                               app.settings.num_rows_to_display,
                                                               app.file_info.has_headers);
                            app.app = AppState::Viewer;
                        }
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
                    if let Some(path) = FileDialog::new().pick_file() {
                        app.settings.current_pos = 0;
                        app.file_path = Option::from(path.display().to_string());
                        app.file_info.total_rows = get_row_count(app.file_path
                            .clone());
                        let mut reader: Reader<File> = get_reader_from_file(app.file_path.clone());
                        app.headers = get_headers_from_file(reader.borrow_mut());
                        app.records = get_records_from_pos(app.file_path.clone(),
                                                           app.settings.current_pos.clone(),
                                                           app.settings.num_rows_to_display,
                                                           app.file_info.has_headers);
                        app.app = AppState::Viewer;

                    }
                }
                // Export Changes to file
                if ui.button("(TBA)Export to...").clicked() {
                    // code here
                }
                // Closes the frame and ends the application.
                if ui.button("Close").clicked() {
                    app.headers = StringRecord::new();
                    app.records = Vec::new();
                    app.file_path = None;
                    app.file_info = FileInfo::default();
                    app.settings = AppSettings::default();
                    app.app = AppState::MainMenu;
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
                if ui.button("(TBA)Sort...").clicked() {
                    // code here
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
                if ui.button("Next Page").clicked() {
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
                if ui.button("Previous Page").clicked() {
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
                if ui.button("Go To Start of File").clicked() {
                    app.settings.current_pos = 0;
                    app.records = get_records_from_pos(
                        app.file_path.clone(),
                        app.settings.current_pos.clone(),
                        app.settings.num_rows_to_display.clone(),
                        app.file_info.has_headers);
                }
                if ui.button("Go To End of File").clicked() {
                    if app.file_info.total_rows > app.settings.num_rows_to_display {
                        app.settings.current_pos = app.file_info.total_rows - app.settings.num_rows_to_display;
                        app.records = get_records_from_pos(
                            app.file_path.clone(),
                            app.settings.current_pos.clone(),
                            app.settings.num_rows_to_display.clone(), app.file_info.has_headers);
                    } else {
                        app.settings.dialog_open = true;
                    }
                }
            });
        });
    });

    // Central Panel. Displays the Cells.
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::horizontal().always_show_scroll(true).stick_to_bottom(true).show(ui, |ui| {
            use egui_extras::{Size, StripBuilder};
            StripBuilder::new(ui)
                .size(Size::remainder().at_least(100.0)) // for the table
                .size(Size::exact(10.0)) // for the source code link
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::horizontal().show(ui, |ui| {
                            // Table Builder
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

fn show_next_page() {

}

fn show_prev_page() {

}

fn show_line() {

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
        println!("Reading from Terminal Stdin");
        let mut reader: Reader<Stdin> = get_reader_stdin();
        // let headers = get_headers_stdin(reader.borrow_mut());
        // let records = get_records_stdin(reader.borrow_mut());
        viewer_app = ViewerApp {
            app: AppState::Viewer,
            file_info: Default::default(),
            headers: get_headers_stdin(reader.borrow_mut()),
            records: get_records_stdin(reader.borrow_mut()),
            file_path: None,
            settings: Default::default(),
        };
    }

    let mut eframe_options = eframe::NativeOptions::default();
    eframe_options.default_theme = Theme::Dark;

    eframe::run_native(
        "CSV Viewer",
        eframe_options,
        Box::new(|cc| Box::new(viewer_app)),
    )
}