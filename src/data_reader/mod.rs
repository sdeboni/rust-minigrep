pub mod file_reader;
pub mod in_memory_reader;

pub trait DataReader {
    fn next(&mut self) -> Option<String>;
}
