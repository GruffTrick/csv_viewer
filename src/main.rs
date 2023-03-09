use std::borrow::{BorrowMut};
use std::process::exit;
use std::clone;
use csv::StringRecord;
use csv_viewer::*;


fn main() {
    let mut reader = get_reader_stdin();

    let mut headers: StringRecord = get_headers(reader.borrow_mut());

    let mut records = get_records(reader.borrow_mut());

    run_app(headers, records, None).expect("Panic: Empty records");
}
