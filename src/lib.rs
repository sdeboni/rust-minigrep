mod search_params;

#[derive(Debug, PartialEq)]
enum DataSource {
    FilePath(String),
    MemorySource(MemorySource),
}

#[derive(Debug, PartialEq)]
struct MemorySource {
    data: Vec<String>,
    idx: usize,
}

#[derive(Debug, PartialEq)]
struct SearchParams {
    query: String,
    data: DataSource,
}

pub enum Error {
    Client(String),
}
