#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{BufRead, Stdin};
use rfd::FileDialog;
use atty;
use csv::{Reader, StringRecord};

use csv_viewer::*;

fn main() {

    // checks if run from terminal without a file passed in
    if atty::isnt(atty::Stream::Stdin){
        // Open from passed file
        println!("Reading from Terminal Stdin");
        let mut reader: Reader<Stdin> = get_reader_stdin();
        let headers:StringRecord = get_headers_stdin(reader.borrow_mut());
        let records: Vec<StringRecord> = get_records_stdin(reader.borrow_mut());
        run_app(headers, records, None).expect("Panic: Empty records");
    } else {
        // Open from file
        if let Some(path) = FileDialog::new().pick_file() {
            let file_path = Some(path.display().to_string());
            let mut reader:Reader<File> = get_reader_file(file_path.clone());
            let headers: StringRecord= get_headers_file(reader.borrow_mut());
            let records: Vec<StringRecord> = get_records_file(reader.borrow_mut());
            run_app(headers, records, None).expect("Panic: Empty records");
        }
    }

}
