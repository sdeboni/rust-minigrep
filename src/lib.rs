mod data_reader;
mod search_params;

use search_params::SearchParamsBuilder;

pub enum Error {
    Client(String),
    Server(String),
}

fn search(args: &[String]) {
    let _ = SearchParamsBuilder::new().args(args).build();
}
