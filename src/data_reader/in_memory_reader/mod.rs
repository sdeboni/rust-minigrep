use crate::data_reader::DataReader;

pub struct InMemoryReader {
    data: Vec<String>,
    idx: usize,
}

impl DataReader for InMemoryReader {
    fn next(&mut self) -> Option<String> {
        if self.idx < self.data.len() {
            let next = &self.data[self.idx];
            self.idx += 1;
            return Some(next.clone());
        }
        None
    }
}

impl InMemoryReader {
    pub fn new(data: Vec<String>) -> InMemoryReader {
        InMemoryReader { data, idx: 0 }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn retrieves_in_memory_data() {
        let mut data_reader = InMemoryReader {
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
        let mut data_reader = InMemoryReader {
            data: vec![],
            idx: 0,
        };

        if let Some(line) = data_reader.next() {
            panic!("{}", format!("expected end of data, got: {}", line));
        }
    }
}
