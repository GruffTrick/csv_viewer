pub mod sort {
    use std::borrow::Borrow;
    use csv::{Position, ReaderBuilder, StringRecord, WriterBuilder};
    use serde::{Deserialize, Serialize};
    use std::error::Error;
    use std::fs::{File, OpenOptions, remove_file};
    use std::vec::IntoIter;
    use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
    use sysinfo::{System, SystemExt};


    /// Sorts the records from the data stored at `file_path` and exports sorted data to `output_path`
    ///
    /// ```
    /// // create a temporary input file
    ///     let input_content = "Name, Age, Gender\nAlice, 25, Female\nBob, 30, Male\nEve, 22, Female\n";
    ///     let input_file = tempfile::NamedTempFile::new()?;
    ///     let input_path = input_file.path().to_str().unwrap().to_owned();
    ///     std::fs::write(&input_path, input_content)?;
    ///
    ///     // create a temporary output file
    ///     let output_file = tempfile::NamedTempFile::new()?;
    ///     let output_path = output_file.path().to_str().unwrap().to_owned();
    ///
    ///     // sort the records by the "Age" column
    ///     sort_records(input_path, output_path.clone(), 1)?;
    ///
    ///     // read the output file
    ///     let mut rdr = csv::Reader::from_path(output_path)?;
    ///     let mut records = rdr.records();
    ///
    ///     // check the order of the records
    ///     assert_eq!(records.next().unwrap()?, ["Eve", "22", "Female"]);
    ///     assert_eq!(records.next().unwrap()?, ["Alice", "25", "Female"]);
    ///     assert_eq!(records.next().unwrap()?, ["Bob", "30", "Male"]);
    ///     assert!(records.next().is_none());
    /// ```
    pub fn sort_records(file_path: String, output_path: String, field_index: usize) -> Result<(), Box<dyn Error>> {
        match remove_file(output_path.clone()) {
            Ok(()) => println!("File successfully deleted."),
            Err(e) => println!("File not found: {}", output_path),
        }

        // Open the CSV file
        let file = OpenOptions::new().read(true).write(true).open(file_path.clone())?;
        let file_size = file.metadata()?.len();
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
        let mut h_test = ReaderBuilder::new().has_headers(true).from_path(file_path.clone())?;

        // Create a new sorted CSV file
        let sorted_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_path)?;
        let mut wtr = WriterBuilder::new().has_headers(true).from_writer(sorted_file);

        // Write the CSV header to the new file
        let header = rdr.headers()?;
        wtr.write_byte_record(&header.as_byte_record())?;

        // Calculate the chunk size based on available memory and file size
        let available_memory = System::new_all().available_memory();
        let chunk_size = (available_memory / 4) as usize;
        let num_chunks = (file_size as usize / chunk_size) + 1;

        // new header to reference, workaround to avoid duplicating the header inside the file.
        let h = h_test.headers()?;

        // Sort the file in chunks
        for chunk_index in 0..num_chunks {
            // Seek to the beginning of the chunk
            let chunk_start = chunk_index * chunk_size;
            // let mut position: Position = Position::new();
            // position.set_record(chunk_start as u64);
            // rdr.seek(position.clone())?;
            // println!("{:?}", position);


            // Read the chunk into memory
            let mut chunk: Vec<StringRecord> = Vec::new();

            for row in 0..chunk_size {
                let mut record: StringRecord = StringRecord::new();
                match rdr.read_record(&mut record) {
                    Ok(false) => break,
                    Ok(_) => {
                        if !matches(&record, &h) {
                            chunk.push(record);
                        }
                    },
                    Err(e) => println!("Error: Cannot Read Chunk"),
                }
            }


            // Sort the records by field index
            chunk.sort_by_key(|record| record.get(field_index).unwrap().to_string());

            // Write the sorted chunk to the new file
            for record in chunk.into_iter() {
                wtr.write_record(record.into_iter())?;
            }
            wtr.flush()?;
        }

        Ok(())
    }

    fn matches(record: &StringRecord, header: &StringRecord) -> bool {
        if record == header { return true };
        false
    }
}

#[cfg(tests)]
mod tests {
    use csv::ReaderBuilder;

    #[test]
    fn test_sort_records() -> Result<(), Box<dyn Error>> {
        use std::error::Error;
        use std::fs::File;
        use std::io::prelude::*;
        use csv::{ReaderBuilder, WriterBuilder, StringRecord};

        // create the file and writer.
        let file = File::create("tests/test_sort.txt")?;
        let mut csv_writer = WriterBuilder::new().has_headers(true).from_writer(file);

        csv_writer.write_record(&[City,State,Population,Latitude,Longitude])?;
        csv_writer.write_record(&["Sandfort", "AL", "", "32.3380556", "-85.2233333"]);
        csv_writer.write_record(&["Shadow Oaks Addition", "AR", "", "34.9555556", "-91.9475000"]);
        csv_writer.write_record(&["Selma", "AL", "18980", "32.4072222", "-87.0211111"]);
        csv_writer.write_record(&["Richards Crossroads", "AL", "", "31.7369444", "-85.2644444"])?;

        let file = File::open("tests/test_sort_output.txt")?;
        let reader = BufReader::new(file);
        let mut records: Vec<StringRecord> = Vec::new();

        // iterate over the records
        let mut csv_reader = csv::ReaderBuilder::new().has_headers(true).from_reader(reader);
        for result in csv_reader.records() {
            let record = result?;
            records.push(record);
        }

        let sorted_records = vec![["Richards Crossroads", "AL", "", "31.7369444", "-85.2644444"],["Sandfort", "AL", "", "32.3380556", "-85.2233333"],["Selma", "AL", "18980", "32.4072222", "-87.0211111"],["Shadow Oaks Addition", "AR", "", "34.9555556", "-91.9475000"]];

        sort_records(String::from("tests/test_sort.txt"), String::from("tests/test_sort_output.txt"), 0);

        assert_eq!(sorted_records, records);

        Ok(())
    }
}