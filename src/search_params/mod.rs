use crate::{DataSource, Error, MemorySource, SearchParams};

pub struct SearchParamsBuilder {
    args: Option<Vec<String>>,
    memory_data_source: Option<Vec<String>>,
}

impl SearchParamsBuilder {
    pub fn new() -> SearchParamsBuilder {
        SearchParamsBuilder {
            args: None,
            memory_data_source: None,
        }
    }

    fn args(mut self, args: &[String]) -> SearchParamsBuilder {
        self.args = Some(args.to_vec());
        self
    }

    fn memory_data_source(mut self, data: &[String]) -> SearchParamsBuilder {
        self.memory_data_source = Some(data.to_vec());
        self
    }

    fn build(self) -> Result<SearchParams, Error> {
        match self.args {
            Some(a) => {
                if a.len() == 2 {
                    return Ok(SearchParams {
                        query: a[0].to_string(),
                        data: DataSource::FilePath(a[1].to_string()),
                    });
                }
                Err(Error::Client("blah".to_string()))
            }
            None => Err(Error::Client("blah".to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_file_path_search_args() {
        let result = SearchParamsBuilder::new()
            .args(&["arg1".to_string(), "arg2".to_string()])
            .build();

        let expected_search_params = SearchParams {
            query: "arg1".to_string(),
            data: DataSource::FilePath("arg2".to_string()),
        };
        match result {
            Ok(search_params) => assert_eq!(expected_search_params, search_params),
            Err(Error::Client(err)) => {
                panic!("{}", format!("got unexpected error: {}", err.to_string()))
            }
        }
    }
}
