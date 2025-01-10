use crate::value::Value;

use super::{AccessError, Register};

/// A basic register simply holds a [`Value`], with methods to read/write said [`Value`].
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct BasicRegister {
    id: String,
    value: Option<Value>,
}

impl BasicRegister {
    /// Returns a register with a given id and an [`Option::None`] value.
    #[must_use]
    pub fn new(id: &str) -> Self {
        BasicRegister {
            id: id.to_string(),
            value: None,
        }
    }

    /// Returns a register with a given id and [`Value`].
    ///
    /// # Errors
    ///
    /// * `NumberValueTooSmall` - if given value is a number less than -9999.
    /// * `NumberValueTooLarge` - if given value is a number greater than 9999.
    /// * `WriteWithLabelId` - if given value is a [`Value::LabelId`].
    /// * `WriteWithRegisterId` - if given value is a [`Value::RegisterId`].
    pub fn new_with_value(id: &str, value: &Value) -> Result<Self, AccessError> {
        let mut register = Self::new(id);

        register.write(value).map(|()| register)
    }
}

impl Register for BasicRegister {
    /// Returns the clone of the register's [`Value`].
    ///
    /// # Errors
    ///
    /// N/A
    fn read(&self) -> Result<Option<Value>, AccessError> {
        Ok(self.value.clone())
    }

    /// Returns the clone of the register's [`Value`] and clears.
    ///
    /// # Errors
    ///
    /// N/A
    fn read_mut(&mut self) -> Result<Option<Value>, AccessError> {
        let result = Ok(self.value.clone());

        self.clear();

        result
    }

    /// Write a given [`Value`] to the register.
    ///
    /// If there is an error, the register will be unchanged.
    ///
    /// # Errors
    ///
    /// * If the given value is a [`Value::LabelId`] or [`Value::RegisterId`].
    /// * If the given value is a [`Value::Number`] not within the [-9999, 9999] bounds.
    fn write(&mut self, value: &Value) -> Result<(), AccessError> {
        match value {
            Value::Number(number) if *number < -9_999 => {
                Err(AccessError::NumberValueTooSmall(value.clone()))
            }
            Value::Number(number) if 9_999 < *number => {
                Err(AccessError::NumberValueTooLarge(value.clone()))
            }
            Value::LabelId(_) => Err(AccessError::WriteWithLabelId(value.clone())),
            Value::RegisterId(_) => Err(AccessError::WriteWithRegisterId(value.clone())),
            _ => {
                self.value = Some(value.clone());

                Ok(())
            }
        }
    }

    /// Clears a register's value.
    ///
    /// Just sets the value to [`Option::None`].
    fn clear(&mut self) {
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_number() {
        let value = Value::from(666);

        let register = BasicRegister::new_with_value("X", &value).unwrap();

        assert_eq!(register.read(), Ok(Some(value)));
        assert!(register.value.is_some());
    }

    #[test]
    fn test_read_keyword() {
        let value = Value::Keyword(String::from("keyword"));

        let register = BasicRegister::new_with_value("X", &value).unwrap();

        assert_eq!(register.read(), Ok(Some(value)));
        assert!(register.value.is_some());
    }

    #[test]
    fn test_read_mut_number() {
        let value = Value::from(666);

        let mut register = BasicRegister::new_with_value("X", &value).unwrap();

        assert_eq!(register.read_mut(), Ok(Some(value)));
        assert!(register.value.is_none());
    }

    #[test]
    fn test_read_mut_keyword() {
        let value = Value::Keyword(String::from("keyword"));

        let mut register = BasicRegister::new_with_value("X", &value).unwrap();

        assert_eq!(register.read_mut(), Ok(Some(value)));
        assert!(register.value.is_none());
    }

    #[test]
    fn test_write_with_number() {
        let mut register = BasicRegister::new("X");

        let value = Value::Number(666);

        let expected_register = BasicRegister {
            id: String::from("X"),
            value: Some(value.clone()),
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_with_number_too_small_err() {
        let mut register = BasicRegister::new_with_value("X", &Value::Number(666)).unwrap();
        let value = Value::Number(-10_000);

        let expected_register = BasicRegister {
            id: String::from("X"),
            value: Some(Value::Number(666)),
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert_eq!(result, Err(AccessError::NumberValueTooSmall(value)));
    }

    #[test]
    fn test_write_with_number_too_large_err() {
        let mut register = BasicRegister::new_with_value("X", &Value::Number(666)).unwrap();
        let value = Value::Number(10_000);

        let expected_register = BasicRegister {
            id: String::from("X"),
            value: Some(Value::Number(666)),
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert_eq!(result, Err(AccessError::NumberValueTooLarge(value)));
    }

    #[test]
    fn test_write_with_keyword() {
        let mut register = BasicRegister::new("X");
        let value = Value::Keyword(String::from("keyword"));

        let expected_register = BasicRegister {
            id: String::from("X"),
            value: Some(value.clone()),
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_with_label_id_err() {
        let mut register = BasicRegister::new("X");
        let value = Value::LabelId(String::from("LABEL"));

        let expected_register = BasicRegister {
            id: String::from("X"),
            value: None,
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert_eq!(result, Err(AccessError::WriteWithLabelId(value)));
    }

    #[test]
    fn test_write_with_register_id_err() {
        let mut register = BasicRegister::new("X");
        let value = Value::RegisterId(String::from("#NERV"));

        let expected_register = BasicRegister {
            id: String::from("X"),
            value: None,
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert_eq!(result, Err(AccessError::WriteWithRegisterId(value)));
    }

    #[test]
    fn test_clear() {
        let mut register =
            BasicRegister::new_with_value("X", &Value::Keyword(String::from("keyword"))).unwrap();

        let expected_register = BasicRegister {
            id: String::from("X"),
            value: None,
        };

        register.clear();

        assert_eq!(register, expected_register);
    }
}
