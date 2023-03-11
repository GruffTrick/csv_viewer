# CSV Viewer
Lightweight CSV Viewer for use in Linux.
Written in Rust.


# Dependencies
Linux has the following packages required:
```
sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev
```

# Running the Application
Until the release version is available, compiling is required first using
```
cargo build
```

Then you can either run the native binary and open from file dialogue
*or*
Run the binary inside the repository directory and pass the csv file to quickly open a file.
```
./target/debug/CSV-Viewer < "CSVFILENAME.csv"
```


## Resources Used:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community:
- https://doc.rust-lang.org/book

CSV Crate Cookbook
- https://docs.rs/csv/latest/csv/cookbook/index.html
