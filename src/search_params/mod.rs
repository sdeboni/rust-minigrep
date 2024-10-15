use crate::{DataSource, Error, FileData, InMemoryData, SearchParams};

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

    fn args(mut self, args: &[String]) -> SearchParamsBuilder {
        self.args = Some(args.to_vec());
        self
    }

    fn in_memory_data(mut self, data: Vec<String>) -> SearchParamsBuilder {
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
                data: DataSource::InMemory(InMemoryData { data: dat, idx: 0 }),
            }),
            None => Err(Error::Client(
                "no file path or in-memory data provided".to_string(),
            )),
        }
    }

    fn build(self) -> Result<SearchParams, Error> {
        match self.args {
            Some(a) => match a.len() {
                2 => Ok(SearchParams {
                    query: a[0].clone(),
                    data: DataSource::File(FileData::new(&a[1])?),
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
                if let DataSource::File(file_data) = search_params.data {
                    assert_eq!(path, file_data.path);
                } else {
                    panic!("expected data_source to be of file type");
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
                if let DataSource::InMemory(in_memory) = search_params.data {
                    assert_eq!(2, in_memory.data.len());
                } else {
                    panic!("expected InMemory data source");
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
