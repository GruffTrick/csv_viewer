#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod viewer_app;
mod reader;
mod sort;
mod find;

use serde::de::Unexpected::Option;
use viewer_app::run_app;
use sort::sort_records;


fn main() {
    // let file_path = "test_sort.csv";
    
    run_app().expect("Runtime Error");
}
