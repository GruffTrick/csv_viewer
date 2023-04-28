#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::{Borrow, BorrowMut};
use std::fs;
use std::fs::{File, read};
use std::io::{Stdin, BufReader, BufRead, Seek};
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
pub fn get_headers_from_file(reader: &mut Reader<File>) -> StringRecord {
    let mut reader = reader;

    let headers = reader.headers().cloned().expect("Panic: No Headers");
    println!("{:?}", headers);

    headers
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
pub fn get_row_count(file_path: Option<String>) -> u64 {
    let mut number_of_rows: u64 = 0;

    let mut record = StringRecord::new();

    // Wrap the file in a buffered reader
    let mut reader = get_reader_from_file(file_path);

    loop {
        match reader.read_record(&mut record) {
            Ok(false) => break, // end of file
            Ok(_) => {
                // Process the data in the buffer
                println!("{:?}", record.clone());
                number_of_rows = number_of_rows + 1;
            }
            Err(e) => println!("Error Reading Rows"), // handle the error
        }
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
pub fn get_records_from_pos(file_path: Option<String>, pos: u64, num_of_rows_to_display: u64, has_header: bool) -> Vec<StringRecord> {
    let mut records: Vec<StringRecord> = Vec::new();
    let mut record = StringRecord::new();
    // let p: String = file_path.unwrap();

    let mut reader = csv::ReaderBuilder::new().has_headers(has_header).from_path(file_path.unwrap()).unwrap();

    let mut start_pos = 0;
    if has_header == true {
        start_pos = 1;
    }

    // Seek to the row of pos
    for row in start_pos..pos+1 {
        match reader.read_record(&mut record) {
            Ok(false) => break,
            Ok(_) => { println!("{:?}", record.clone()) },
            Err(e) => println!("TODO: Error"),
        }
    }

    // Read data from the file into records as a StringRecord Vector
    for row in 0..num_of_rows_to_display {
        match reader.read_record(&mut record) {
            Ok(false) => break, // end of file
            Ok(_) => {
                // Process the data in the buffer
                println!("{:?}", record.clone());
                records.push(record.clone())
            }
            Err(e) => println!("TODO: Error"), // handle the error
        }
    }

    records
}

