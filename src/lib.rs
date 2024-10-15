use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;

mod search_params;

trait DataReader {
    fn next(&mut self) -> Option<String>;
}

enum DataSource {
    File(FileData),
    InMemory(InMemoryData),
}

struct FileData {
    path: String,
    lines: std::iter::Flatten<io::Lines<io::BufReader<File>>>,
}

impl FileData {
    fn new(path: &str) -> Result<FileData, Error> {
        match read_lines(path) {
            Ok(lines) => Ok(FileData {
                path: path.to_string(),
                lines: lines.flatten(),
            }),
            Err(err) => Err(Error::Client(format!("failed to open {}: {}", path, err))),
        }
    }

    fn next(mut self) -> Option<String> {
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

#[derive(Debug, PartialEq)]
struct InMemoryData {
    data: Vec<String>,
    idx: usize,
}

impl DataReader for InMemoryData {
    fn next(&mut self) -> Option<String> {
        if self.idx < self.data.len() {
            let next = &self.data[self.idx];
            self.idx += 1;
            return Some(next.clone());
        }
        None
    }
}

struct SearchParams {
    query: String,
    data: DataSource,
}

pub enum Error {
    Client(String),
    Server(String),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn retrieves_in_memory_data() {
        let mut data_reader = InMemoryData {
            data: vec!["line1".to_string(), "line2".to_string()],
            idx: 0,
        };

        match data_reader.next() {
            Some(line) => assert_eq!("line1", line),
            None => panic!("expected line1, got end of data"),
        }

        match data_reader.next() {
            Some(line) => assert_eq!("line2", line),
            None => panic!("expected line2, got end of data"),
        }

        if let Some(line) = data_reader.next() {
            panic!("{}", format!("expected end of data, got: {}", line));
        }
    }

    #[test]
    fn handles_empty_in_memory_data() {
        let mut data_reader = InMemoryData {
            data: vec![],
            idx: 0,
        };

        if let Some(line) = data_reader.next() {
            panic!("{}", format!("expected end of data, got: {}", line));
        }
    }
}
