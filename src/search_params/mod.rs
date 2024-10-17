use crate::data_reader::file_reader::FileData;
use crate::data_reader::in_memory_reader::InMemoryData;
use crate::data_reader::DataReader;

use crate::Error;

pub struct SearchParams {
    query: String,
    data: Box<dyn DataReader>,
}

pub struct SearchParamsBuilder {
    args: Option<Vec<String>>,
    in_memory_data: Option<Vec<String>>,
}

impl SearchParamsBuilder {
    pub fn new() -> SearchParamsBuilder {
        SearchParamsBuilder {
            args: None,
            in_memory_data: None,
        }
    }

    pub fn args(mut self, args: &[String]) -> SearchParamsBuilder {
        self.args = Some(args.to_vec());
        self
    }

    pub fn in_memory_data(mut self, data: Vec<String>) -> SearchParamsBuilder {
        self.in_memory_data = Some(data);
        self
    }

    fn get_search_parms_for_in_memory_data(
        query: String,
        data: Option<Vec<String>>,
    ) -> Result<SearchParams, Error> {
        match data {
            Some(dat) => Ok(SearchParams {
                query,
                data: Box::new(InMemoryData::new(dat)),
            }),
            None => Err(Error::Client(
                "no file path or in-memory data provided".to_string(),
            )),
        }
    }

    pub fn build(self) -> Result<SearchParams, Error> {
        match self.args {
            Some(a) => match a.len() {
                2 => Ok(SearchParams {
                    query: a[0].clone(),
                    data: Box::new(FileData::new(&a[1])?),
                }),
                1 => SearchParamsBuilder::get_search_parms_for_in_memory_data(
                    a[0].clone(),
                    self.in_memory_data,
                ),
                _ => Err(Error::Client(format!(
                    "expeced 1 or 2 arguments, got {}",
                    a.len()
                ))),
            },
            None => Err(Error::Client("blah".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_file_path_search_args() {
        let path = "/home/sdeboni/projects/minigrep/test.dat".to_string();

        let result = SearchParamsBuilder::new()
            .args(&["query".to_string(), path.clone()])
            .build();

        match result {
            Ok(search_params) => {
                assert_eq!("query", search_params.query);
                let mut reader: Box<dyn DataReader> = search_params.data;

                match reader.next() {
                    Some(line) => assert_eq!("file_line1", line),
                    None => panic!("unexpeced EOF"),
                }
            }
            Err(Error::Client(err)) => {
                panic!("{}", format!("got unexpected client error: {}", err))
            }
            Err(Error::Server(err)) => {
                panic!("{}", format!("got unexpected server error: {}", err))
            }
        }
    }

    #[test]
    fn handles_stub_data() {
        let result = SearchParamsBuilder::new()
            .args(&["query".to_string()])
            .in_memory_data(vec!["line1".to_string(), "line2".to_string()])
            .build();

        match result {
            Ok(search_params) => {
                assert_eq!("query", search_params.query);
                let mut reader: Box<dyn DataReader> = search_params.data;

                match reader.next() {
                    Some(line) => assert_eq!("line1", line),
                    None => panic!("unexpeced EOF"),
                }
            }
            Err(Error::Client(err)) => {
                panic!("{}", format!("got unexpected client error: {}", err))
            }
            Err(Error::Server(err)) => {
                panic!("{}", format!("got unexpected server error: {}", err))
            }
        }
    }
}
