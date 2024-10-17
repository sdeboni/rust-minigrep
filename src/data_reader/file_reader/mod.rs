use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use crate::data_reader::DataReader;
use crate::Error;

pub struct FileData {
    path: String,
    lines: std::iter::Flatten<io::Lines<io::BufReader<File>>>,
}

impl FileData {
    pub fn new(path: &str) -> Result<FileData, Error> {
        match read_lines(path) {
            Ok(lines) => Ok(FileData {
                path: path.to_string(),
                lines: lines.flatten(),
            }),
            Err(err) => Err(Error::Client(format!("failed to open {}: {}", path, err))),
        }
    }
}

impl DataReader for FileData {
    fn next(&mut self) -> Option<String> {
        self.lines.next()
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
