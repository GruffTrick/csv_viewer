use std::borrow::BorrowMut;
use std::clone;
use std::io::BufRead;
use csv::StringRecord;
use rfd::FileDialog;
use atty;

use csv_viewer::*;

fn main() {

    // checks if run from terminal without a file passed in
    if atty::isnt(atty::Stream::Stdin){
        println!("Reading from Terminal Stdin");
        let mut reader = get_reader_stdin();
        let headers = get_headers_stdin(reader.borrow_mut());
        let records = get_records_stdin(reader.borrow_mut());
        run_app(headers, records, None).expect("Panic: Empty records");
    } else { // Open from file
        if let Some(path) = FileDialog::new().pick_file() {
            let file_path = Some(path.display().to_string());
            let mut reader = get_reader_file(file_path.clone());
            let headers = get_headers_file(reader.borrow_mut());
            let records = get_records_file(reader.borrow_mut());
            run_app(headers, records, None).expect("Panic: Empty records");
        }
    }

}
