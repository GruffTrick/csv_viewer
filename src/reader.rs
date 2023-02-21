use std::io;

fn read_from_file() {
    let mut rdr = csv::Reader::from_reader(io::stdin());

    for result in rdr.records() {
        let record = result.expect("a csv record");

        println!("{:?}", record)
    }
}