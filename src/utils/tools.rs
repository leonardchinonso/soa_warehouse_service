use crate::errors::app_error::AppError;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

// generate_random_alphanum generates random alphanumeric characters of a given size
pub fn generate_random_alphanum(size: usize) -> Result<String, AppError> {
    const RAND_SIZE: usize = 30;
    if size > RAND_SIZE {
        return Err(AppError::new(format!(
            "size must not be greater than {}",
            RAND_SIZE
        )));
    }

    // generate random string with size from thread_rng
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();

    Ok(rand_string)
}

// split_into_parts splits a string into parts of a given size and returns the string
pub fn split_into_parts(str: String, size: usize) -> String {
    let mut res = "".to_owned();

    for (i, c) in str.chars().enumerate() {
        if i % size == 0 && i != 0 {
            res.push('-')
        }
        res.push(c)
    }

    return res;
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use actix_web::test;

    use super::generate_random_alphanum;
    use super::split_into_parts;

    #[test]
    async fn test_split_into_parts() {
        let test_cases = vec![
            ("tictactoe", 3, "tic-tac-toe"),
            ("abcdefghijk", 4, "abcd-efgh-ijk"),
        ];

        for test_case in test_cases {
            let got = split_into_parts(test_case.0.to_string(), test_case.1);
            assert_eq!(got, test_case.2)
        }
    }

    #[test]
    // test_generate_random_alphanum_returns_err tests that generate_random_alphanum returns a string
    async fn test_generate_random_alphanum_returns_str() {
        let test_cases = vec![2, 5, 6, 0, 30];

        for test_case in test_cases {
            let got = generate_random_alphanum(test_case);
            assert_eq!(got.unwrap().len(), test_case);
        }
    }

    #[test]
    // test_generate_random_alphanum_returns_err tests that generate_random_alphanum returns an error
    async fn test_generate_random_alphanum_returns_err() {
        let test_cases = vec![31, 32, 40];

        for test_case in test_cases {
            let got = generate_random_alphanum(test_case);
            assert!(got.is_err());
            assert_eq!(
                got.unwrap_err().description(),
                "size must not be greater than 30"
            );
        }
    }
}
