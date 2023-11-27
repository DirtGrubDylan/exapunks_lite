use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::convert::{From, Into};
use std::str::FromStr;

/// A `Value` is used to hold several types of information: number, keyword, register id, and a
/// label id. Each type is used by an [`Exa`] to perform their tasks. Whether it is storing keywords
/// from a [`Register`] to a [`File`], or asking their [`Program`] to jump to a specific label. Or
/// even adding two numbers and storing them to a [`Register`] with a specified id.
///
/// An [`Instruction`] cannot have keyword values, but they can have source/destination register ids
/// and numbers.
///
/// A [`Register`] can hold on to a number or keyword value.
#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord)]
pub enum Value {
    Number(isize),
    Keyword(String),
    RegisterId(String),
    LabelId(String),
}

/// A dummy struct to indicate that there was an error on the [`FromStr`] implementations.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ValueParseError;

impl From<isize> for Value {
    fn from(input: isize) -> Self {
        Value::Number(input)
    }
}

impl Into<isize> for Value {
    fn into(self) -> isize {
        match self {
            Self::Number(number) => number,
            _ => panic!("Cannot convert {self:?} into an isize!"),
        }
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::Number(number) => number.to_string(),
            Self::Keyword(keyword) => keyword.clone(),
            Self::RegisterId(register_id) => register_id.clone(),
            Self::LabelId(label_id) => label_id.clone(),
        }
    }
}

impl FromStr for Value {
    type Err = ValueParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<isize>() {
            _ if s.is_empty() => Err(ValueParseError),
            Ok(number) => Ok(Value::Number(number)),
            Err(_) => Ok(Value::Keyword(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Value;

    #[test]
    fn test_parse_from_str_to_number() {
        let number_string = "-127".to_string();

        let number = number_string.parse().unwrap();

        assert_eq!(Value::Number(-127), number);
    }

    #[test]
    fn test_parse_from_str_to_keyword() {
        let keyword_string: String = "keyword".to_string();

        let keyword = keyword_string.parse().unwrap();

        assert_eq!(Value::Keyword("keyword".to_string()), keyword);
    }

    #[test]
    fn test_parse_from_empty_str_err() {
        let empty_string = String::new();

        let value_err = empty_string.parse::<Value>();

        assert!(value_err.is_err());
    }

    #[test]
    fn test_from_isize_to_number() {
        let number = Value::Number(-127);

        let new_number: Value = Value::from(-127);

        assert_eq!(number, new_number);
    }

    #[test]
    fn test_to_from_number_and_isize() {
        let number = Value::Number(-127);

        let number_value: isize = number.into();

        assert_eq!(-127, number_value);
    }

    #[test]
    #[should_panic]
    fn test_from_non_number_to_isize_panics() {
        let keyword = Value::Keyword("keyword".to_string());

        let _to_isize: isize = keyword.into();
    }

    #[test]
    fn test_from_number_to_string() {
        let number = Value::Number(-127);

        let number_string: String = number.to_string();

        assert_eq!("-127".to_string(), number_string);
    }

    #[test]
    fn test_from_non_number_to_string() {
        let keyword = Value::Keyword("keyword".to_string());

        let keyword_string: String = keyword.to_string();

        assert_eq!("keyword".to_string(), keyword_string);
    }
}
