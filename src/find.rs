use std::fs::File;
use std::io::{BufRead, BufReader};

use csv::StringRecord;

pub fn find_matching_rows(file_path:Option<String>, search_str: String, has_headers: bool) -> Vec<usize> {
    let mut matched_rows: Vec<usize> = Vec::new();
    let file = File::open(file_path.unwrap()).unwrap();
    let reader = BufReader::new(file);
    let mut row_pos = 0;

    for line in reader.lines() {
        let row = line.unwrap();
        let row_bytes = row.as_bytes();

        // iterates through the file finding matching lines,
        // adding index of the row to a vector.
        for window in row_bytes.windows(search_str.as_bytes().len()) {

            if window == search_str.as_bytes() {
                matched_rows.push(row_pos);
                break;
            }
        }
        row_pos = row_pos + 1;
    }
    matched_rows
}

pub fn find_row_of_next(vec: Vec<usize>, i: usize) -> usize {
    let mut match_index: usize = 0;
    if i < vec.len() {
        match_index = vec[i];
    }
    match_index
}