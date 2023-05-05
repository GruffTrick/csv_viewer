pub mod viewer_app;
pub mod reader;
pub mod sort;
pub mod find;

use viewer_app::viewer_app::run_app;


fn main() {

    run_app().expect("Runtime Error");
}
