#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::Borrow;
use csv::{Position, ReaderBuilder, StringRecord, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions, remove_file};
use std::vec::IntoIter;
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};
use sysinfo::{System, SystemExt};

pub fn _sort_records(column_index: usize) -> Result<(), Box<dyn Error>> {
    let file_path = "sorted_data.csv"; // replace with argument
    let is_head = true;


    match remove_file(file_path) {
        Ok(()) => println!("File successfully deleted."),
        Err(e) => println!("Error deleting file: {}", e),
    }

    // Open the CSV file
    let file = OpenOptions::new().read(true).write(true).open("sort_test.csv")?;
    let file_size = file.metadata()?.len();
    let mut rdr = ReaderBuilder::new().has_headers(is_head).from_reader(file);

    // Create a new sorted CSV file
    let sorted_file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open("sorted_data.csv")?;
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(sorted_file);

    // Write the CSV header to the new file
    let header = rdr.headers().cloned().expect("Couldn't Extract Headers");
    wtr.write_byte_record(&header.as_byte_record())?;

    // Calculate the chunk size based on available memory and file size
    let available_memory = System::new_all().available_memory();
    let chunk_size = (available_memory / 4) as usize;
    let num_chunks = (file_size as usize / chunk_size) + 1;

    // Sort the file in chunks
    for chunk_index in 0..num_chunks {
        // Seek to the beginning of the chunk
        let chunk_start = chunk_index * chunk_size;
        let mut position: Position = Position::new();
        position.set_record(chunk_start as u64);
        rdr.seek(position.clone())?;
        println!("{:?}", position);


        // Read the chunk into memory
        let mut chunk: Vec<StringRecord> = Vec::new();

        for row in 0..chunk_size {
            let mut record: StringRecord = Default::default();
            match rdr.read_record(&mut record) {
                Ok(false) => break,
                Ok(_) => { chunk.push(record) },
                Err(e) => println!("Error: Cannot Read Chunk"),
            }
        }


        // Sort the records by field index
        chunk.sort_by_key(|record| record.get(column_index).unwrap().to_string());

        // Write the sorted chunk to the new file
        for record in chunk {
            wtr.write_record(record.into_iter())?;
        }
        wtr.flush()?;
    }



    Ok(())
}



// pub fn extern_sort(filename: &str, max_mem_use: usize) {
//     let file = BufReader::with_capacity(BUFFER_CAPACITY, File::open(filename).unwrap());
//     let mut v = vec![];
//     let mut tmp_file_names = vec![];
//     for x in file.lines() {
//         v.push(x.unwrap().parse::<f64>().unwrap());
//         if mem::size_of::<f64>() * v.len() > max_mem_use {
//             sort_and_write_to_file(&mut v, &mut tmp_file_names);
//         }
//     }
//     if v.len() > 0 {
//         sort_and_write_to_file(&mut v, &mut tmp_file_names);
//     }
//     merge(&tmp_file_names, filename);
//     clean_up(&tmp_file_names);
// }
