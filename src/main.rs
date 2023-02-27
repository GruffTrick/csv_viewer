use std::process::exit;
use csv::StringRecord;
use csv_viewer::read_from_file;


fn main() -> eframe::Result<()> {
    let v: Vec<StringRecord> = read_from_file();

    // run_app(v);

    exit(1)
}
