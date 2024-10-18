mod data_reader;
mod search_params;

use crate::search_params::SearchParamsBuilder;
use std::env;
use std::fmt;

pub enum AppError {
    Client(String),
    Server(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Client(err) => write!(f, "Client Error: {}", err),
            AppError::Server(err) => write!(f, "Server Error: {}", err),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match SearchParamsBuilder::new().args(&args[1..]).build() {
        Ok(mut search) => {
            let mut out: Vec<String> = vec![];
            search.exec(&mut out);

            for line in out {
                println!("{}", line);
            }
        }
        Err(err) => eprintln!("{}", err),
    }
}
