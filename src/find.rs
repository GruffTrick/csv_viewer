pub mod find {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    /// Finds all matches and returns a vector of the indices of rows with matching strings.
    ///# Example
    /// This test creates a CSV file with 5 rows, and then calls the `find_matching_rows` function
    ///to search for rows that contain the string "30". The function should return the indices of the
    /// first and last rows. This is because they both contain the string "30".
    /// ```
    /// use csv_viewer::find::find::find_matching_rows;
    ///
    /// // "Alice,30\nBob,35\nCharlie,25\nDave,40\nEve,30\n"
    /// let file_path = "tests/test_find_matching.csv";
    /// let contents = "Alice,30\nBob,35\nCharlie,25\nDave,40\nEve,30\n";
    /// std::fs::write(file_path, contents).unwrap();
    ///
    /// let search_str = "30";
    /// let result = find_matching_rows(Some(file_path.to_owned()), search_str.to_owned(), true);
    ///
    /// // matching row indices should be 0 and 4.
    /// assert_eq!(result, vec![0, 4]);
    /// ```

    pub fn find_matching_rows(
        file_path: Option<String>,
        search_str: String,
        has_headers: bool,
    ) -> Vec<usize> {
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

    /// Returns the index of the next matching string, stored at within the vector
    /// using the index i for the element within the vec.
    ///
    /// # Example
    ///```
    /// use csv_viewer::find::find::find_row_of_next;
    ///
    /// let vec = vec![1, 2, 3, 4, 5];
    ///
    /// assert_eq!(find_row_of_next(vec, 0), 1);
    /// assert_eq!(find_row_of_next(vec, 3), 4);
    /// assert_eq!(find_row_of_next(vec, 4), 0);
    /// ```
    pub fn find_row_of_next(vec: Vec<usize>, i: usize) -> usize {
        let mut match_index: usize = 0;
        if i < vec.len() {
            match_index = vec[i];
        }
        match_index
    }
}
