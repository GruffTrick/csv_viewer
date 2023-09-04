# CSV Viewer
Lightweight CSV Viewer for use in Linux.
Written in Rust.


## Dependencies
The following packages required to run on linux:
```
sudo apt-get install -y libclang-dev libgtk-3-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libspeechd-dev libxkbcommon-dev libssl-dev
```

## Running the Application
The binary is located within the `/target/release` directory.  As long as dependencies are installed, can be run by right clicking and pressing run, or can be run from the terminal.


### Resources Used:

The Rust Programming Language
by Steve Klabnik and Carol Nichols, with contributions from the Rust Community:
- https://doc.rust-lang.org/book

CSV Crate Cookbook
- https://docs.rs/csv/latest/csv/cookbook/index.html


### Third Party Libraries
`csv` crate by BurntSushi
avaiable at: https://github.com/BurntSushi/rust-csv`

`egui` and `eframe` by emilk
available at: https://github.com/emilk/egui

`atty` by softprops
avaiable at:https://github.com/softprops/atty

`rfd` by PolyMeilex
available at: https://github.com/PolyMeilex/rfd
