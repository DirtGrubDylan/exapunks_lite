use std::fs::File;
use std::io::{BufRead, BufReader};

/// Takes a file and returns it's contents as a vector of strings.
///
/// # Errors
///
/// IF something happens while reading an inidividual line.
///
/// # Panics
///
/// If line couldn't be read and/or file doesn't exist.
pub fn to_string_vector(file_name: &str) -> Result<Vec<String>, String> {
    let file = BufReader::new(File::open(file_name).expect("File not found!"));

    Ok(file
        .lines()
        .map(|line| line.expect("The file is bad!"))
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string_vector() {
        let expected = vec![
            String::from("LINK 800"),
            String::new(),
            String::from("COPY 4 X"),
            String::new(),
            String::from("# Loop a few times"),
            String::from("MARK THIS_LABEL"),
            String::from("SUBI X 1 X"),
            String::from("TEST X = 0"),
            String::from("FJMP THIS_LABEL"),
            String::new(),
            String::from("HALT"),
        ];

        let result = to_string_vector("test_files/simple_program.exa").unwrap();

        assert_eq!(result, expected);
    }
}
