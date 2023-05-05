#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::{Borrow, BorrowMut};
use std::fs;
use std::fs::{File, read};
use std::io::{Stdin, BufReader, BufRead, Seek, Read, Cursor};
use std::path::Path;
use std::error::Error;
use std::io;
use std::mem::size_of_val;
use csv::{Position, Reader, ReaderBuilder, StringRecord};


/// Returns a reader object from stdin input
pub fn get_reader_stdin() -> Reader<io::Stdin> {
    let mut reader = Reader::from_reader(io::stdin());
    reader
}

/// Returns a reader object from a File path
pub fn get_reader_from_file(p: Option<String>) -> Reader<File> {
    let p = p.unwrap();
    let mut reader = Reader::from_path(p);
    reader.unwrap()
}

/// Extracts and returns the headers from a file-read reader object
pub fn get_headers_from_file(file_path: String) -> StringRecord {
    let file = File::open(file_path).unwrap();
    let mut reader = BufReader::new(file);
    let mut header_reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(&mut reader);
    let header = header_reader.headers().unwrap().clone();

    header
}

/// Extracts and returns the records from a file-read reader object
pub fn get_records_file(reader: &mut Reader<File>) -> Vec<StringRecord> {
    let mut reader= reader;
    let mut records: Vec<StringRecord> = Vec::new();

    for result in reader.records() {
        let record = result.expect("a csv record");
        // println!("{:?}", record);
        records.push(record)
    }
    println!("{:?}", records);
    println!("Size of records in memory: {:?}bytes", size_of_val(&records));
    records
}

/// Extracts and returns the headers from a stdin-read reader object
pub fn get_headers_stdin(reader: &mut Reader<Stdin>) -> StringRecord {
    let mut reader = reader;

    let headers = reader.headers().cloned().expect("Panic: Expected headers");
    println!("{:?}", headers);

    headers
}

/// Extracts and returns the records from a file-read reader object
pub fn get_records_stdin(reader: &mut Reader<Stdin>) -> Vec<StringRecord> {
    let reader= reader;
    let mut records: Vec<StringRecord> = Vec::new();

    for result in reader.records() {
        let record = result.expect("a csv record");
        // println!("{:?}", record);
        records.push(record)
    }
    println!("{:?}", records);
    println!("Size of records in memory: {:?}bytes", size_of_val(&records));
    records
}


/// Returns the row count of the file.
pub fn get_row_count(file_path: Option<String>) -> usize {
    // let mut number_of_rows: u64 = 0;
    //
    // let mut record = StringRecord::new();
    //
    // // Wrap the file in a buffered reader
    // let mut reader = get_reader_from_file(file_path);
    //
    // loop {
    //     match reader.read_record(&mut record) {
    //         Ok(false) => break, // end of file
    //         Ok(_) => {
    //             // Process the data in the buffer
    //             println!("{:?}", record.clone());
    //             number_of_rows = number_of_rows + 1;
    //         }
    //         Err(e) => println!("Error Reading Rows"), // handle the error
    //     }
    // }
    let file = File::open(file_path.unwrap()).unwrap();
    let reader = BufReader::new(file);


    let mut number_of_rows = 0;
    for _ in reader.lines() {
        number_of_rows += 1;
    }

    number_of_rows
}

fn _get_file_size_mb(file_path: String) -> f64 {
    let metadata = fs::metadata(file_path).unwrap();
    let size_in_bytes = metadata.len();
    let size_in_mb = size_in_bytes as f64 / (1024.0 * 1024.0);
    // println!("Size of file: {:.2} MB", size_in_mb);
    size_in_mb
}

/// Builds a vector of String Records by reading a buffer of pre-determined size
/// from the referenced file path.
/// ```
/// let file_path = "uspop.csv";
/// let pos = 6;
/// let rows_to_display = 4;
/// let test_records: Vec<StringRecord> = [[Richards Crossroads,AL,,31.7369444,-85.2644444],[Sandfort,AL,,32.3380556,-85.2233333],[Selma,AL,18980,32.4072222,-87.0211111],[Shadow Oaks Addition,AR,,34.9555556,-91.9475000]]
///
/// let result_records = doccomments::get_records_from_pos(file_path, pos, rows_to_display, true);
/// asserteq!(result_records, test_records);
/// ```
pub fn get_records_from_pos(file_path: Option<String>, pos: usize, num_of_rows_to_display: usize, has_header: bool) -> Vec<StringRecord> {
    let file = File::open(file_path.unwrap()).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    let mut lines_read = 0;

    // skip header
    if has_header && (pos == 0) {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        lines_read = 1
    }

    // create CSV reader
    let mut csv_reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(!has_header)
        .from_reader(reader);

    // skip to starting position
    for _ in 0..pos {
        let mut record = StringRecord::new();
        let bytes_read = csv_reader.read_record(&mut record).unwrap();
        if bytes_read == false {
            break;
        }
    }

    // read line forward by 1 if file has header and not reading from pos 0
    if has_header && (pos != 0) {
        let mut record = StringRecord::new();
        csv_reader.read_record(&mut record).unwrap();
    }

    // read records into buffer
    for result in csv_reader.records() {
        if let Ok(record) = result {
            buffer.push(record.clone());
            if buffer.len() >= num_of_rows_to_display {
                break;
            }
        }
    }
    buffer
}
