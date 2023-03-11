use std::borrow::{Borrow, BorrowMut};
use std::fs::{File, read};
use std::io;
use std::io::Stdin;
use std::path::Path;
use std::error::Error;
use csv::{Reader, StringRecord};


pub fn get_reader_stdin() -> Reader<io::Stdin> {
    let mut reader = Reader::from_reader(io::stdin());
    reader
}

pub fn get_reader_file(p: Option<String>) -> Reader<File> {
    let mut p = p.unwrap();
    let mut reader = Reader::from_path(p);
    reader.unwrap()
}

pub fn get_headers_file(reader: &mut Reader<File>) -> StringRecord {
    let mut reader = reader;

    let headers = reader.headers().cloned().expect("Panic: No Headers");
    println!("{:?}", headers);

    headers
}

pub fn get_records_file(reader: &mut Reader<File>) -> Vec<StringRecord> {
    let mut reader= reader;
    let mut records: Vec<StringRecord> = Vec::new();

    for result in reader.records() {
        let record = result.expect("a csv record");
        // println!("{:?}", record);
        records.push(record)
    }
    println!("{:?}", records);

    records
}


pub fn get_headers_stdin(reader: &mut Reader<Stdin>) -> StringRecord {
    let mut reader = reader;

    let headers = reader.headers().cloned().expect("Panic: Expected headers");
    println!("{:?}", headers);

    headers
}

pub fn get_records_stdin(reader: &mut Reader<Stdin>) -> Vec<StringRecord> {
    let mut reader= reader;
    let mut records: Vec<StringRecord> = Vec::new();

    for result in reader.records() {
        let record = result.expect("a csv record");
        // println!("{:?}", record);
        records.push(record)
    }
    println!("{:?}", records);

    records
}

