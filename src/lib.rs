#[derive(Debug, PartialEq)]
struct Search {
    query: String,
    path: String,
}

pub enum Error {
    Client(String),
}

fn parse_input(input: &[String]) -> Result<Search, Error> {
    let expected = 2;
    if input.len() != expected {
        Err(Error::Client(format!(
            "Expected {} arguments found {}",
            expected,
            input.len()
        )))
    } else {
        let query = input[0].to_string();
        let path = input[1].to_string();

        Ok(Search { query, path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_error_when_insufficient_args() {
        let expected_arg_count = 2;
        let too_few_args = expected_arg_count - 1;

        let mut input_with_too_few_args = vec![];
        for n in 0..too_few_args {
            input_with_too_few_args.push(format!("arg{}", n));
        }
        assert!(expected_arg_count > input_with_too_few_args.len());

        let result = parse_input(&input_with_too_few_args);

        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Client(err) => {
                assert!(err.contains(&(format!("{}", expected_arg_count))));
                assert!(err.contains(&(format!("{}", input_with_too_few_args.len()))));
            }
        }
    }

    #[test]
    fn returns_error_when_too_many_args() {
        let expected_arg_count = 2;
        let too_many_arg_count = expected_arg_count + 1;

        let mut input_with_too_many_args = vec![];
        for n in 0..too_many_arg_count {
            input_with_too_many_args.push(format!("arg{}", n));
        }
        assert!(expected_arg_count < input_with_too_many_args.len());

        let result = parse_input(&input_with_too_many_args);
        assert!(result.is_err());
        match result.unwrap_err() {
            Error::Client(err) => {
                assert!(err.contains(&(format!("{}", expected_arg_count))));
                assert!(err.contains(&(format!("{}", input_with_too_many_args.len()))));
            }
        }
    }
    #[test]
    fn parses_arguments() {
        let expected_query = "arbitrary_query".to_string();
        let expected_path = "arbitrary_path".to_string();
        let input = vec![expected_query.clone(), expected_path.clone()];

        let expected_search = Search {
            query: expected_query.clone(),
            path: expected_path.clone(),
        };

        let result = parse_input(&input);

        match result {
            Ok(actual_search) => {
                assert_eq!(expected_search, actual_search);
            }
            Err(_) => panic!("unexpected error"),
        }
    }
}
