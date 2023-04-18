use std::borrow::{Borrow, BorrowMut};
use std::fs;
use std::fs::{File, read};
use std::io::{Stdin, BufReader, BufRead, Seek};
use std::path::Path;
use std::error::Error;
use std::io;
use std::mem::size_of_val;
use csv::{Position, Reader, StringRecord};

const MAX_BUF_SIZE: usize = 1_000_000;


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




pub fn get_row_count(file_path: Option<String>) -> u64 {
    let mut number_of_rows: u64 = 0;

    let mut record = StringRecord::new();

    // Wrap the file in a buffered reader
    let mut reader = get_reader_from_file(file_path);

    loop {
        // match reader.read_line(&mut buffer) {
        //     Ok(0) => break, // end of file
        //     Ok(_) => {
        //         // Process the data in the buffer
        //         // ...
        //         println!("Row: {}, Content: {}", number_of_rows +1, buffer);
        //         number_of_rows = number_of_rows + 1;
        //         buffer.clear(); // clear the buffer for the next chunk
        //     }
        //     Err(e) => print!("Error") // handle the error
        // }
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

fn get_file_size_mb(file_path: String) -> f64 {
    let metadata = fs::metadata(file_path).unwrap();
    let size_in_bytes = metadata.len();
    let size_in_mb = size_in_bytes as f64 / (1024.0 * 1024.0);
    // println!("Size of file: {:.2} MB", size_in_mb);
    size_in_mb
}

/// Builds a vector of String Records by reading a buffer of pre-determined size
/// from the referenced file path.
pub fn get_records_from_pos(file_path: Option<String>, pos: u64, num_of_rows_to_display: u64) -> Vec<StringRecord> {
    let mut records: Vec<StringRecord> = Vec::new();
    let mut record = StringRecord::new();

    // Wrap the file in a buffered reader
    let mut reader = get_reader_from_file(file_path);

    for row in 1..pos {
        match reader.read_record(&mut record) {
            Ok(false) => break,
            Ok(_) => { println!("{:?}", record.clone()) },
            Err(e) => println!("TODO: Error"),
        }
    }

    // Read data from the file into the buffer
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

