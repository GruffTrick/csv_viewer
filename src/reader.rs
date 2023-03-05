use std::io;
use std::path::Path;
use csv::{Reader, StringRecord};


pub fn read_from_stdin() -> Vec<StringRecord> {
    let mut reader = Reader::from_reader(io::stdin());
    let mut v: Vec<StringRecord> = Vec::new();

    {
        let headers = reader.headers();
        println!("{:?}", headers);
    }

    for result in reader.records() {
        let record = result.expect("a csv record");
        // println!("{:?}", record);
        v.push(record)
    }
    println!("{:?}", v);
    v
}


pub fn read_from_path(p: &Path) -> Vec<StringRecord> {
    let mut _reader = Reader::from_path(p);
    let mut v: Vec<StringRecord> = Vec::new();

    v
}