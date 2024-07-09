pub mod sort {
    use csv::{ReaderBuilder, StringRecord, WriterBuilder};
    use std::borrow::Borrow;
    use std::error::Error;
    use std::fs::{remove_file, File, OpenOptions};
    use std::io::{Write};
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
    ///
    pub fn sort_records(
        file_path: String,
        output_path: String,
        field_index: usize,
    ) -> Result<(), Box<dyn Error>> {
        match remove_file(output_path.clone()) {
            Ok(()) => println!("File successfully deleted."),
            Err(e) => println!("File not found: {}", output_path),
        }

        // Open the CSV file
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(file_path.clone())?;
        let file_size = file.metadata()?.len();
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);
        let mut h_test = ReaderBuilder::new()
            .has_headers(true)
            .from_path(file_path.clone())?;

        // Create a new sorted CSV file
        let sorted_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(output_path)?;
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .from_writer(sorted_file);

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
                    }
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

    /// Checks whether two string records match one another.  Intended for comparing a passed string record value
    /// to the header, to avoid printing the header twice.
    ///
    /// # Example
    /// ```
    /// use csv_viewer::sort::sort::matches;
    /// use csv::StringRecord;
    ///
    /// let header = StringRecord::from(vec!["name", "age", "city"]);
    /// let record = StringRecord::from(vec!["Gruff Trick", "22", "Wales"]);
    /// let match_result = matches(&record, &header);
    /// assert_eq!(match_result, false);
    ///
    /// let header = StringRecord::from(vec!["name", "age", "city"]);
    /// let record = StringRecord::from(vec!["name", "age", "city"]);
    /// let match_result = matches(&record, &header);
    /// assert_eq!(match_result, true);
    /// ```
    fn matches(record: &StringRecord, header: &StringRecord) -> bool {
        if record == header {
            return true;
        };
        false
    }
}