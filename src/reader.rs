pub mod reader {
    use std::borrow::{Borrow, BorrowMut};
    use std::fs;
    use std::fs::{File, read};
    use std::io::{Stdin, BufReader, BufRead, Seek, Read, Cursor};
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
    pub fn get_headers_from_file(file_path: String, delimiter: char) -> StringRecord {
        let file = File::open(file_path).unwrap();
        let mut reader = BufReader::new(file);
        let mut header_reader = ReaderBuilder::new().delimiter(u8::try_from(delimiter).unwrap())
            .has_headers(false)
            .from_reader(&mut reader);
        let header = header_reader.headers().unwrap().clone();

        header
    }

    /// Extracts and returns the records from a file-read reader object
    pub fn get_records_file(reader: &mut Reader<File>) -> Vec<StringRecord> {
        let mut reader = reader;
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
        let reader = reader;
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
    ///
    /// # Examples
   ///  ```
   ///  use std::fs::File;
   ///  use std::io::{BufWriter, Write};
   ///
   ///  let file = File::create("tests/test_row_count.txt");
   ///  let mut writer = BufWriter::new(&mut file).unwrap();
   ///
   ///  // Write 10 lines of data to the file
   ///  for i in 1..=10 {
   ///      let line = format!("Line {}\n", i);
   ///      writer.write(line.as_bytes()).unwrap();
   ///  }
   ///  writer.flush();
   ///
   ///  let row_count_result = csv_viewer::reader::reader::get_row_count(Option::from(String::from("tests/test_row_count.txt")));
   ///  assert_eq!(row_count_result, 10);
   /// ```
    ///
    pub fn get_row_count(file_path: Option<String>) -> usize {
        let file = File::open(file_path.unwrap()).unwrap();
        let reader = BufReader::new(file);


        let mut number_of_rows = 0;
        for _ in reader.lines() {
            number_of_rows += 1;
        }

        number_of_rows
    }


    /// Gets the size of the file in megabytes to 2 decimal places.
    ///
    /// # Example
    /// ```
    /// let file_path = String::from("tests/test_filesize.txt");
    /// let size = csv_viewer::reader::reader::_get_file_size_mb(file_path);
    /// println!("File size: {:.2} MB", size);
    /// ```
    pub fn _get_file_size_mb(file_path: String) -> f64 {
        let metadata = fs::metadata(file_path).unwrap();
        let size_in_bytes = metadata.len();
        let size_in_mb = size_in_bytes as f64 / (1024.0 * 1024.0);
        // println!("Size of file: {:.2} MB", size_in_mb);
        size_in_mb
    }

    /// Builds a vector of String Records by reading a buffer of pre-determined size
    /// from the referenced file path.
    ///
   ///  # Examples
    ///
   /// ```
   /// use csv::StringRecord;
   /// let file_path = Option::from(String::from("test_uspop.csv"));
   /// let pos = 3; // starts at index 0, skips one line for header
   /// let rows_to_display = 4;
   /// let test_records: Vec<StringRecord> = vec![
   ///     StringRecord::from(vec!["Richards Crossroads", "AL", "", "31.7369444", "-85.2644444"]),
   ///     StringRecord::from(vec!["Sandfort", "AL", "", "32.3380556", "-85.2233333"]),
   ///     StringRecord::from(vec!["Selma", "AL", "18980", "32.4072222", "-87.0211111"]),
   ///     StringRecord::from(vec!["Shadow Oaks Addition", "AR", "", "34.9555556", "-91.9475000"]),
   /// ];
   ///
   /// let result_records = csv_viewer::reader::reader::get_records_from_pos(file_path, pos, rows_to_display, true);
   /// assert_eq!(test_records, result_records);
   /// ```
    ///
    /// This test reads in four StringRecords from index 3 of `test_uspop.csv`.
    /// These StringRecords are then tested against the actual expected StringRecord to see if the
    /// file is read from correctly.
    ///
    /// ```
    /// let csv_data = "Name, Age, City
    /// Alice, 30, New York
    /// Bob, 25, San Francisco
    /// Charlie, 40, Los Angeles";
    ///         let mut file = Cursor::new(Vec::new());
    ///         file.write_all(csv_data.as_bytes()).unwrap();
    ///
    ///         // Test the function by reading the second record and the header
    ///         let records = get_records_from_pos(Some("test.csv".to_string()), 1, 2, true);
    ///         assert_eq!(records.len(), 2);
    ///         assert_eq!(records[0][0], "Bob");
    ///         assert_eq!(records[1][0], "Charlie");
    ///     }
    /// ```
    pub fn get_records_from_pos(file_path: Option<String>, pos: usize, num_of_rows_to_display: usize, has_header: bool, delimiter: char) -> Vec<StringRecord> {
        let file = File::open(file_path.unwrap()).unwrap();
        let mut reader = BufReader::new(file);
        let mut buffer = Vec::new();
        let mut lines_read = 0;

        // skip header
        if has_header && (pos == 0) {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            lines_read = 1
        }

        // create CSV reader+


        let mut csv_reader = ReaderBuilder::new()
            .delimiter(u8::try_from(delimiter).unwrap())
            .has_headers(!has_header)
            .from_reader(reader);

        // skip to starting position
        for _ in 0..pos {
            let mut record = StringRecord::new();
            let bytes_read = csv_reader.read_record(&mut record).unwrap();
            if bytes_read == false {
                break;
            }
        }

        // read line forward by 1 if file has header and not reading from pos 0
        if has_header && (pos != 0) {
            let mut record = StringRecord::new();
            csv_reader.read_record(&mut record).unwrap();
        }

        // read records into buffer
        for result in csv_reader.records() {
            if let Ok(record) = result {
                buffer.push(record.clone());
                if buffer.len() >= num_of_rows_to_display {
                    break;
                }
            }
        }
        buffer
    }
}