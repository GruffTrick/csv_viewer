use std::io;
use csv::StringRecord;

pub fn read_from_file() -> Vec<StringRecord> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut v: Vec<StringRecord> = Vec::new();

    for result in rdr.records() {
        let record = result.expect("a csv record");
        // println!("{:?}", record);
        v.push(record)
    }
    println!("{:?}", v);
    v
}