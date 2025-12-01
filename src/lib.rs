use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn read_input(day: u8) -> Vec<String> {
    let file_path = PathBuf::from(format!("input/{day}.input"));
    let file = File::open(file_path).unwrap_or_else(|_| panic!("missing file {day}"));
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.expect("Invalid input formatting"))
        .collect()
}
