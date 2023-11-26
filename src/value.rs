use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use std::convert::{From, Into};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign};

/// A `Value` is used to hold several types of information: number, keyword, register id, and a
/// label id. Each type is used by an [`Exa`] to perform their tasks. Whether it is storing keywords
/// from a [`Register`] to a [`File`], or asking their [`Program`] to jump to a specific label. Or
/// even adding two numbers and storing them to a [`Register`] with a specified id.
///
/// An [`Instruction`] cannot have keyword values, but they can have source/destination register ids
/// and numbers.
///
/// A [`Register`] can hold on to a number or keyword value.
#[derive(Debug)]
pub enum Value {
    Number(isize),
    Keyword(String),
    RegisterId(String),
    LabelId(String),
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(lhs), Value::Number(rhs)) => lhs == rhs,
            (Value::Keyword(lhs), Value::Keyword(rhs)) => lhs == rhs,
            _ => panic!("Cannot check equivalence for {self:?} and {other:?}!"),
        }
    }
}

impl Eq for Value {}

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

#[cfg(test)]
mod tests {
    use super::Value;

    #[test]
    fn test_new() {
        unimplemented!()
    }

    #[test]
    fn test_add_number() {
        unimplemented!()
    }

    #[test]
    fn test_add_non_number_panics() {
        unimplemented!()
    }

    #[test]
    fn test_subtract_number() {
        unimplemented!()
    }

    #[test]
    fn test_subtract_non_number_panics() {
        unimplemented!()
    }

    #[test]
    fn test_divide_number() {
        unimplemented!()
    }

    #[test]
    fn test_divide_non_number_panics() {
        unimplemented!()
    }

    #[test]
    fn test_multiply_number() {
        unimplemented!()
    }

    #[test]
    fn test_multiply_non_number_panics() {
        unimplemented!()
    }

    #[test]
    fn test_modulo_number() {
        unimplemented!()
    }

    #[test]
    fn test_modulo_non_number_panics() {
        unimplemented!()
    }

    #[test]
    fn test_greater_than_number() {
        unimplemented!()
    }

    #[test]
    fn test_greater_than_keyword() {
        unimplemented!()
    }

    #[test]
    fn test_greater_than_non_number_non_keyword_panics() {
        unimplemented!()
    }

    #[test]
    fn test_less_than_number() {
        unimplemented!()
    }

    #[test]
    fn test_less_than_keyword() {
        unimplemented!()
    }

    #[test]
    fn test_less_than_non_number_non_keyword_panics() {
        unimplemented!()
    }

    #[test]
    fn test_equal_to_number() {
        let first = Value::Number(128);
        let second = Value::Number(128);
        let third = Value::Number(-128);

        assert_eq!(first, second);
        assert_ne!(first, third);
    }

    #[test]
    fn test_equal_to_keyword() {
        let first = Value::Keyword("first".to_string());
        let second = Value::Keyword("first".to_string());
        let third = Value::Keyword("third".to_string());

        assert_eq!(first, second);
        assert_ne!(first, third);
    }

    #[test]
    #[should_panic]
    fn test_equal_to_non_number_non_keyword_panics() {
        let first = Value::Keyword("first".to_string());
        let second = Value::Number(128);

        assert_eq!(first, second);
    }

    #[test]
    fn test_from_str_to_number() {
        unimplemented!()
    }

    #[test]
    fn test_from_str_to_keyword() {
        unimplemented!()
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
    fn test_from_non_number_to_isize_panics() {
        unimplemented!()
    }

    #[test]
    fn test_from_number_to_string() {
        unimplemented!()
    }

    #[test]
    fn test_from_non_number_to_string() {
        unimplemented!()
    }
}
