use crate::{DataSource, Error, InMemoryData, SearchParams};

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
                    data: DataSource::FilePath(a[1].clone()),
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
        let expected_search_params = SearchParams {
            query: "arg1".to_string(),
            data: DataSource::FilePath("arg2".to_string()),
        };

        let result = SearchParamsBuilder::new()
            .args(&["arg1".to_string(), "arg2".to_string()])
            .build();

        match result {
            Ok(search_params) => assert_eq!(expected_search_params, search_params),
            Err(Error::Client(err)) => {
                panic!("{}", format!("got unexpected error: {}", err.to_string()))
            }
        }
    }

    #[test]
    fn handles_stub_data() {
        let expected_search_params = SearchParams {
            query: "query".to_string(),
            data: DataSource::InMemory(InMemoryData {
                data: vec!["line1".to_string(), "line2".to_string()],
                idx: 0,
            }),
        };

        let result = SearchParamsBuilder::new()
            .args(&["query".to_string()])
            .in_memory_data(vec!["line1".to_string(), "line2".to_string()])
            .build();

        match result {
            Ok(search_params) => assert_eq!(expected_search_params, search_params),
            Err(Error::Client(err)) => {
                panic!("{}", format!("got unexpected error: {}", err.to_string()))
            }
        }
    }
}
