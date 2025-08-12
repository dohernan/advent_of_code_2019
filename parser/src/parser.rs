use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Parser {}

impl Parser {
    pub fn from_txt(path: &Path) -> Vec<i64> {
        let file = File::open(path).expect("File coudlnt be opened");
        let reader = BufReader::new(file);

        let mut data = Vec::new();

        for line in reader.lines() {
            let line = line.expect("Cannot read line");
            data.push(line.parse().expect("Cannot parse number"));
        }

        data
    }

    pub fn from_txt_signed(path: &Path) -> Vec<i64> {
        println!("{}", path.display());
        let file = File::open(path).expect("File coudlnt be opened");
        let reader = BufReader::new(file);

        let mut data = Vec::new();

        for line in reader.lines() {
            let line = line.expect("Cannot read line");
            data.push(line.parse().expect("Cannot parse number"));
        }

        data
    }
}
