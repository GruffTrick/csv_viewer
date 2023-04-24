#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use csv::StringRecord;

pub fn _sort_records(v: &Vec<StringRecord>, column: usize) -> Vec<StringRecord> {
    let mut v = Vec::new();
    let mut row: usize;
    let mut col: usize;



    // record sorting functionality here:
    //
    //      Will use a match case most likely to detect the column's type
    //      Then Sort using appropriate search function
    //      Likely QuickSort or HeapSort (nlogn)
    //      Dependant on size of data

    return v
}
