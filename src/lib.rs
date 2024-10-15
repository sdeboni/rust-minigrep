mod search_params;

#[derive(Debug, PartialEq)]
enum DataSource {
    FilePath(String),
    InMemory(InMemoryData),
}

#[derive(Debug, PartialEq)]
struct InMemoryData {
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
