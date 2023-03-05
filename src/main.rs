use std::process::exit;
use csv::StringRecord;
use csv_viewer::*;


fn main() {
    let mut reader = get_reader_stdin();
    let mut headers: Vec<StringRecord> = Vec::new();
    let mut records = get_records(reader);

    run_app(headers, records).expect("Panic: Empty records");
}
