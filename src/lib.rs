#[derive(Debug)]
pub struct ClientError(String);

impl std::error::Error for ClientError {}
impl std::fmt::Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ClientError {
    fn from(msg: String) -> Self {
        Self(msg)
    }
}

pub struct Search {
    pub query: String,
    pub path: String,
}

pub enum Errors {
    Client(ClientError),
}

pub fn parse_input(input: &[String]) -> Result<Search, Errors> {
    if input.len() < 2 {
        return Err(Errors::Client(ClientError::from(
            "Expected 2 arguments".to_string(),
        )));
    }
    let query = input[0].to_string();
    let path = input[1].to_string();

    Ok(Search { query, path })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_test() {
        let expected_query = "arbitrary_query".to_string();
        let expected_path = "arbitrary_path".to_string();

        let input = vec![expected_query.clone(), expected_path.clone()];

        let result = parse_input(&input);

        assert!(result.is_ok());
    }

    #[test]
    fn returns_error_when_insufficient_args_passed() {
        let input_missing_path = vec!["single_arg".to_string()];

        let result = parse_input(&input_missing_path);

        assert!(result.is_err());
    }
}
