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
//
// pub fn get_reader_path(p: &Path) -> Reader<File> {
//     let mut reader = Reader::from_path(p);
//     let mut v: Vec<StringRecord> = Vec::new();
//
//     reader
// }


pub fn get_headers(reader: &mut Reader<Stdin>) -> StringRecord {
    let mut reader = reader;

    let headers = reader.headers().cloned().expect("Panic: Expected headers");
    println!("{:?}", headers);

    headers
}

pub fn get_records(reader: &mut Reader<Stdin>) -> Vec<StringRecord> {
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

