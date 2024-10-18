use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

use crate::data_reader::DataReader;
use crate::AppError;

pub struct FileReader {
    path: String,
    lines: std::iter::Flatten<io::Lines<io::BufReader<File>>>,
}

impl FileReader {
    pub fn new(path: &str) -> Result<FileReader, AppError> {
        match read_lines(path) {
            Ok(lines) => Ok(FileReader {
                path: path.to_string(),
                lines: lines.flatten(),
            }),
            Err(err) => Err(AppError::Client(format!(
                "failed to open {}: {}",
                path, err
            ))),
        }
    }
}

impl DataReader for FileReader {
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
