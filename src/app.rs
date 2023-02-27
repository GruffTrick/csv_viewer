use std::ffi::OsString;
use std::vec::IntoIter;
use csv::StringRecord;
use crate::read_from_file;


pub const NUM_ROWS: i32 = 100;
pub const NUM_COLUMNS: i32 = 100;

pub struct ViewerApp {
    records: Vec<StringRecord>
}
impl Default for ViewerApp {
    fn default() -> Self {
        Self {
            records: read_from_file(),
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
        let Self { records} = self;

        //#[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // top panel for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        // Central Panel, displays main content of the viewer, the grid of cells.
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui,|ui| {
                egui::Grid::new("some_unique_id").show(ui, |ui| {
                    for (row, record) in records.iter().enumerate() {
                        // println!("In position {} we have value {:?}", row, record);
                        ui.label(format!("In position {} we have value {:?}", row, record));
                        ui.end_row();
                    }
                    // for row in 1..NUM_ROWS {
                    //     for column in 1..NUM_COLUMNS {
                    //         ui.label(format!("{},{}",row,column));
                    //     }
                    //     ui.end_row();
                    // }
                });
            });
        });

        // Bottom panel for displaying contextual info like the debug identifier and coordinates.
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::warn_if_debug_build(ui);
        });

    }

    // Called by the frame work to save current state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }
}
