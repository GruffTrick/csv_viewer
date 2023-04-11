use std::borrow::{Borrow, BorrowMut};
use std::fs::{File, read};
use std::io::{Stdin,BufReader};
use std::path::Path;
use std::error::Error;
use std::io;
use std::mem::size_of_val;
use csv::{Reader, StringRecord};

const MAX_BUF_SIZE: usize = 1_000_000;

/// Returns a reader object from stdin input
pub fn get_reader_stdin() -> Reader<io::Stdin> {
    let mut reader = Reader::from_reader(io::stdin());
    reader
}

/// Returns a reader object from a File path
pub fn get_reader_file(p: Option<String>) -> Reader<File> {
    let mut p = p.unwrap();
    let mut reader = Reader::from_path(p);
    reader.unwrap()
}

/// Extracts and returns the headers from a file-read reader object
pub fn get_headers_file(reader: &mut Reader<File>) -> StringRecord {
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

// fn _get_buf_reader(file_path: String) -> BufReader<File>{
//     let file = File::open(file_path)?;
//     let mut reader = BufReader::new(file);
//
//     reader
// }


/// Reads the next chunk of records to the buffer.
fn _read_next_buf() {

    // Implementation here

}


/// Convert buffer to a Vector of StringRecord type
/// return the String Records.
fn _convert_to_string_record() {

    // Implementation Here

}


fn _buf_read_records() -> Vec<StringRecord> {
    let mut v: Vec<StringRecord> = Vec::new();

    v
}