mod data_reader;
mod search_params;

use search_params::SearchParamsBuilder;
use std::fmt;

pub enum AppError {
    Client(String),
    Server(String),
}

impl fmt::Display for crate::AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Client(err) => write!(f, "Client Error: {}", err),
            AppError::Server(err) => write!(f, "Server Error: {}", err),
        }
    }
}

pub fn search(args: &[String], out: &mut Vec<String>) {
    let search_params = SearchParamsBuilder::new().args(args).build();

    match search_params {
        Ok(mut params) => params.exec(out),
        Err(err) => panic!("{}", err),
    }
}
