# CSV Viewer
Lightweight CSV Viewer for use in Linux.
Currently, being written in Rust.


# Running the Application
Packages required to run on linux:
```
sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev
```

Until the release binary available, running the debug binary from the terminal is required.
Run the binary in target/debug directory of the source code and piping in the csv file is required.
```
./target/debug/CSV-Viewer < "CSVFILENAME.csv"
```




## Resources Used:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community:
- https://doc.rust-lang.org/book

CSV Crate Cookbook
- https://docs.rs/csv/latest/csv/cookbook/index.html
