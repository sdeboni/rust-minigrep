use crate::data_reader::file_reader::FileReader;
use crate::data_reader::in_memory_reader::InMemoryReader;
use crate::data_reader::DataReader;

use crate::AppError;

pub struct SearchParams {
    query: String,
    reader: Box<dyn DataReader>,
}

impl SearchParams {
    pub fn exec(&mut self, out: &mut Vec<String>) {
        if let Some(line) = self.reader.next() {
            if line.contains(&self.query) {
                out.push(line);
            }
        }
    }
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
    ) -> Result<SearchParams, AppError> {
        match data {
            Some(dat) => Ok(SearchParams {
                query,
                reader: Box::new(InMemoryReader::new(dat)),
            }),
            None => Err(AppError::Client(
                "no file path or in-memory data provided".to_string(),
            )),
        }
    }

    pub fn build(self) -> Result<SearchParams, AppError> {
        match self.args {
            Some(a) => match a.len() {
                2 => Ok(SearchParams {
                    query: a[0].clone(),
                    reader: Box::new(FileReader::new(&a[1])?),
                }),
                1 => SearchParamsBuilder::get_search_parms_for_in_memory_data(
                    a[0].clone(),
                    self.in_memory_data,
                ),
                _ => Err(AppError::Client(format!(
                    "expeced 1 or 2 arguments, got {}",
                    a.len()
                ))),
            },
            None => Err(AppError::Client("blah".to_string())),
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
                let mut reader: Box<dyn DataReader> = search_params.reader;

                match reader.next() {
                    Some(line) => assert_eq!("file_line1", line),
                    None => panic!("unexpeced EOF"),
                }
            }
            Err(AppError::Client(err)) => {
                panic!("{}", format!("got unexpected client error: {}", err))
            }
            Err(AppError::Server(err)) => {
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
                let mut reader: Box<dyn DataReader> = search_params.reader;

                match reader.next() {
                    Some(line) => assert_eq!("line1", line),
                    None => panic!("unexpeced EOF"),
                }
            }
            Err(AppError::Client(err)) => {
                panic!("{}", format!("got unexpected client error: {}", err))
            }
            Err(AppError::Server(err)) => {
                panic!("{}", format!("got unexpected server error: {}", err))
            }
        }
    }

    #[test]
    fn outputs_query_match() {
        match SearchParamsBuilder::new()
            .args(&["line1".to_string()])
            .in_memory_data(vec!["line1".to_string(), "line2".to_string()])
            .build()
        {
            Ok(mut search) => {
                let mut out: Vec<String> = vec![];
                search.exec(&mut out);
                assert_eq!(1, out.len());
                assert_eq!("line1", out[0]);
            }
            Err(err) => panic!("{}", err),
        }
    }
}
