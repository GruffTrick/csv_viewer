# CSV Viewer
Lightweight CSV Viewer for use in Linux.
Written in Rust.

![Viewer Screen](/assets/img/viewer.png?raw=true "Viewer Screen")

## Dependencies
The following packages required to run on Linux:
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


### Third-Party Libraries
`csv` crate by BurntSushi
avaiable at: https://github.com/BurntSushi/rust-csv`

`egui` and `eframe` by emilk
available at: https://github.com/emilk/egui

`atty` by softprops
avaiable at:https://github.com/softprops/atty

`rfd` by PolyMeilex
available at: https://github.com/PolyMeilex/rfd
<<<<<<< HEAD


## TODO
 - Finish delimiter selection
 - - delimiter character  automatic detection feature

 - Utilise multithreading
 - - Draw progress bars on main thread
 - - Run functions on spawned threads to remove 


## Changelog

- 1.0.1 - added delimiter selection option in the main menu.

- v1.0.0 - initial public release
=======
>>>>>>> refs/remotes/origin/main
