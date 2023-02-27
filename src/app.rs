use std::vec::IntoIter;
use egui::WidgetType::SelectableLabel;
use serde::de::Unexpected::Str;

pub const NUM_ROWS: i32 = 100;
pub const NUM_COLUMNS: i32 = 100;

/// We derive Deserialize/Serialize so we can persist app state
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ViewerApp {
    label: String,

    #[serde(skip)]
    value: f32,
}

impl Default for ViewerApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
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
        let Self { label, value } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
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

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui,|ui| {
                egui::Grid::new("some_unique_id").show(ui, |ui| {
                    for row in 1..NUM_ROWS {
                        for column in 1..NUM_COLUMNS {
                            ui.label(format!("{},{}",row,column));
                        }
                        ui.end_row();
                    }
                });
            });
        });

        // Disabled until it doesn't obscure the scroll bar.
        // egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
        //     // Bottom panel for displaying contextual info like the debug identifier and coordinates.
        //     egui::warn_if_debug_build(ui);
        // });

    }

    // /// Called by the frame work to save current state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }
}