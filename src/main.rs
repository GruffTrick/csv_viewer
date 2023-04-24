#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use crate::viewer_app::run_app;

mod viewer_app;
mod reader;
mod sort;


fn main() {
    run_app().expect("Runtime Error");
}
