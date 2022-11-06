use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use differ::{Differ, Tag};

fn main() {
    let file_a = File::open(Path::new("md5sums_1.txt")).unwrap();
    let reader_a = BufReader::new(&file_a);

    let file_b = File::open(Path::new("md5sums_1.txt")).unwrap();
    let reader_b = BufReader::new(&file_b);

    let lines_a: Vec<String> = reader_a.lines().collect::<Result<_, _>>().unwrap();
    let lines_b: Vec<String> = reader_b.lines().collect::<Result<_, _>>().unwrap();

    print!("{:?}",lines_a);
    print!("{:?}",lines_b);
}