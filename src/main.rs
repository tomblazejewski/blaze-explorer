use ratatui::widgets::{Row, Table};
use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    let file_name_row = Row::new(paths);
}
