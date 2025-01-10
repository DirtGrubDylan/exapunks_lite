use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::convert::From;
use std::fmt;
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
    /// A number.
    Number(isize),
    /// A keyword.
    Keyword(String),
    /// The Id of a [`Register`].
    RegisterId(String),
    /// The Id of label in the [`Program`].
    LabelId(String),
}

/// A dummy struct to indicate that there was an error on the [`FromStr`] implementation.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParseError;

impl Value {
    /// Tries to return a [`Value::Number`] or [`Value::RegisterId`] from the given input.
    ///
    /// A valid register id is either:
    ///
    /// * A single non-numeric character
    /// * A string that has 5 characters and starts with '#'
    ///
    /// # Errors
    ///
    /// This will error if the input is empty, is not a valid number, or is not a valid register
    /// id.
    ///
    /// # Examples
    ///
    /// ```
    /// let number = "-9999";
    /// let hardware_register_id = "#NERV";
    /// let exa_register_id = "X";
    /// let empty = "";
    ///
    /// let expected_number_result = Ok(Value::Number(-9999));
    /// let expected_hardware_register_id_result = Ok(Value::RegisterId("#NERV".to_string()));
    /// let expected_exa_register_id_result = Ok(Value::RegisterId("X".to_string()));
    ///
    /// let number_result = Value::new_number_or_register_id(number);
    /// let hardware_register_id_result = Value::new_number_or_register_id(hardware_register_id);
    /// let exa_register_id_result = Value::new_number_or_register_id(exa_register_id);
    /// let empty_result = Value::new_number_or_register_id(exa_register_id);
    ///
    /// assert_eq!(number_result, expected_number_result);
    /// assert_eq!(
    ///     hardware_register_id_result,
    ///     expected_hardware_register_id_result
    /// );
    /// assert_eq!(exa_register_id_result, expected_exa_register_id_result);
    /// assert!(empty_result.is_err());
    /// ```
    pub fn new_number_or_register_id(input: &str) -> Result<Self, ParseError> {
        match input.parse::<Value>() {
            Ok(Self::Number(number)) => Ok(Self::Number(number)),
            Ok(Self::Keyword(keyword)) => Self::new_register_id(&keyword),
            _ => Err(ParseError),
        }
    }

    /// Tries to return a [`Value::RegisterId`] from the given input.
    ///
    /// A valid register id is either:
    ///
    /// * A single non-numeric character that is 'X', 'T', 'F', or 'M'
    /// * A string that has 5 characters and starts with '#'
    ///
    /// # Errors
    ///
    /// This will error if the input is empty, is a number, or is not a valid register id.
    ///
    /// # Examples
    ///
    /// ```
    /// let number = "-9999";
    /// let hardware_register_id = "#NERV";
    /// let exa_register_id = "X";
    /// let empty = "";
    ///
    /// let expected_hardware_register_id_result = Ok(Value::RegisterId("#NERV".to_string()));
    /// let expected_exa_register_id_result = Ok(Value::RegisterId("X".to_string()));
    ///
    /// let number_result = Value::new_number_or_register_id(number);
    /// let hardware_register_id_result = Value::new_number_or_register_id(hardware_register_id);
    /// let exa_register_id_result = Value::new_number_or_register_id(exa_register_id);
    /// let empty_result = Value::new_number_or_register_id(exa_register_id);
    ///
    /// assert_eq!(
    ///     hardware_register_id_result,
    ///     expected_hardware_register_id_result
    /// );
    /// assert_eq!(exa_register_id_result, expected_exa_register_id_result);
    /// assert!(empty_result.is_err());
    /// assert!(number_result.is_err());
    /// ```
    pub fn new_register_id(input: &str) -> Result<Self, ParseError> {
        let is_valid_hardware_register_id = input.starts_with('#') && (input.len() == 5);

        let is_valid_exa_register_id = matches!(input, "X" | "T" | "F" | "M");

        if is_valid_hardware_register_id || is_valid_exa_register_id {
            Ok(Value::RegisterId(input.to_string()))
        } else {
            Err(ParseError)
        }
    }

    /// Tries to return a [`Value::LabelId`] from the given input.
    ///
    /// # Errors
    ///
    /// This will error if the input is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// let id = "JUMP_TO_THIS";
    /// let empty = "";
    ///
    /// let expected = Ok(Value::LabelId("JUMP_TO_THIS".to_string()));
    ///
    /// let result = Value::new_label_id(id);
    /// let empty_result = Value::new_label_id(empty);
    ///
    /// assert_eq!(result, expected);
    /// assert!(empty_result.is_err());
    /// ```
    pub fn new_label_id(input: &str) -> Result<Self, ParseError> {
        if input.is_empty() {
            Err(ParseError)
        } else {
            Ok(Value::LabelId(input.to_string()))
        }
    }
}

impl From<isize> for Value {
    fn from(input: isize) -> Self {
        Value::Number(input)
    }
}

impl From<Value> for isize {
    fn from(input: Value) -> isize {
        match input {
            Value::Number(number) => number,
            _ => panic!("Cannot convert {input:?} into an isize!"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_string = match self {
            Self::Number(number) => number.to_string(),
            Self::Keyword(keyword) => keyword.clone(),
            Self::RegisterId(register_id) => register_id.clone(),
            Self::LabelId(label_id) => label_id.clone(),
        };

        write!(f, "{as_string}")
    }
}

impl FromStr for Value {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<isize>() {
            _ if s.is_empty() => Err(ParseError),
            Ok(number) => Ok(Value::Number(number)),
            Err(_) => Ok(Value::Keyword(s.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Value;

    #[test]
    fn test_new_number_or_register_id() {
        let number = "-9999";
        let hardware_register_id = "#NERV";
        let exa_register_id = "X";

        let expected_number_result = Ok(Value::Number(-9999));
        let expected_hardware_register_id_result = Ok(Value::RegisterId("#NERV".to_string()));
        let expected_exa_register_id_result = Ok(Value::RegisterId("X".to_string()));

        let number_result = Value::new_number_or_register_id(number);
        let hardware_register_id_result = Value::new_number_or_register_id(hardware_register_id);
        let exa_register_id_result = Value::new_number_or_register_id(exa_register_id);

        assert_eq!(number_result, expected_number_result);
        assert_eq!(
            hardware_register_id_result,
            expected_hardware_register_id_result
        );
        assert_eq!(exa_register_id_result, expected_exa_register_id_result);
    }

    #[test]
    fn test_new_number_or_register_id_err() {
        let invalid_id1 = "#NERVX";
        let invalid_id2 = "";
        let invalid_id3 = "#";
        let invalid_id4 = "#NER";

        let result1 = Value::new_number_or_register_id(invalid_id1);
        let result2 = Value::new_number_or_register_id(invalid_id2);
        let result3 = Value::new_number_or_register_id(invalid_id3);
        let result4 = Value::new_number_or_register_id(invalid_id4);

        assert!(result1.is_err());
        assert!(result2.is_err());
        assert!(result3.is_err());
        assert!(result4.is_err());
    }

    #[test]
    fn test_new_register_id() {
        let hardware_register_id = "#NERV";
        let exa_register_id = "X";

        let expected_hardware_register_id_result = Ok(Value::RegisterId("#NERV".to_string()));
        let expected_exa_register_id_result = Ok(Value::RegisterId("X".to_string()));

        let hardware_register_id_result = Value::new_register_id(hardware_register_id);
        let exa_register_id_result = Value::new_register_id(exa_register_id);

        assert_eq!(
            hardware_register_id_result,
            expected_hardware_register_id_result
        );
        assert_eq!(exa_register_id_result, expected_exa_register_id_result);
    }

    #[test]
    fn test_new_register_id_err() {
        let invalid_id1 = "#NERVX";
        let invalid_id2 = "1";
        let invalid_id3 = "";
        let invalid_id4 = "#";
        let invalid_id5 = "#NER";
        let invalid_id6 = "N";

        let result1 = Value::new_register_id(invalid_id1);
        let result2 = Value::new_register_id(invalid_id2);
        let result3 = Value::new_register_id(invalid_id3);
        let result4 = Value::new_register_id(invalid_id4);
        let result5 = Value::new_register_id(invalid_id5);
        let result6 = Value::new_register_id(invalid_id6);

        assert!(result1.is_err());
        assert!(result2.is_err());
        assert!(result3.is_err());
        assert!(result4.is_err());
        assert!(result5.is_err());
        assert!(result6.is_err());
    }

    #[test]
    fn test_new_labal_id() {
        let id = "JUMP_TO_THIS";

        let expected = Ok(Value::LabelId("JUMP_TO_THIS".to_string()));

        let result = Value::new_label_id(id);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_new_label_id_err() {
        let invalid_id = "";

        let result = Value::new_label_id(invalid_id);

        assert!(result.is_err());
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
    #[should_panic(expected = "Cannot convert Keyword(\"keyword\") into an isize!")]
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
