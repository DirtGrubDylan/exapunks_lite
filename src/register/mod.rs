use crate::value::Value;

/// A Register simply holds a [`Value`], with methods to read/write said [`Value`].
#[derive(Debug, PartialEq, Clone)]
pub struct Register {
    id: String,
    value: Option<Value>,
}

/// A dummy struct to hold possible register write errors.
#[derive(Debug, PartialEq, Clone)]
pub enum RegisterWriteError {
    NumberValueTooSmall(Value),
    NumberValueTooLarge(Value),
    WriteWithLabelId(Value),
    WriteWithRegisterId(Value),
}

impl Register {
    /// Returns a register with a given id and an [`Option::None`] value.
    #[must_use]
    pub fn new(id: &str) -> Self {
        Register {
            id: id.to_string(),
            value: None,
        }
    }

    /// Returns a register with a given id and [`Value`].
    ///
    /// # Errors
    ///
    /// * If the given value is a [`Value::LabelId`] or [`Value::RegisterId`].
    /// * If the given value is a [`Value::Number`] not within the [-9999, 9999] bounds.
    pub fn new_with_value(id: &str, value: &Value) -> Result<Self, RegisterWriteError> {
        let mut register = Self::new(id);

        register.write(value).map(|()| register)
    }

    /// Returns a register with a given id and [`Value`].
    ///
    /// This will clone the [`Value`] that the register is holding.
    #[must_use]
    pub fn read(&self) -> Option<Value> {
        self.value.clone()
    }

    /// Write a given [`Value`] to the register.
    ///
    /// If there is an error, the register will be unchanged.
    ///
    /// # Errors
    ///
    /// * If the given value is a [`Value::LabelId`] or [`Value::RegisterId`].
    /// * If the given value is a [`Value::Number`] not within the [-9999, 9999] bounds.
    pub fn write(&mut self, value: &Value) -> Result<(), RegisterWriteError> {
        match value {
            Value::Number(number) if *number < -9_999 => {
                Err(RegisterWriteError::NumberValueTooSmall(value.clone()))
            }
            Value::Number(number) if 9_999 < *number => {
                Err(RegisterWriteError::NumberValueTooLarge(value.clone()))
            }
            Value::LabelId(_) => Err(RegisterWriteError::WriteWithLabelId(value.clone())),
            Value::RegisterId(_) => Err(RegisterWriteError::WriteWithRegisterId(value.clone())),
            _ => {
                self.value = Some(value.clone());

                Ok(())
            }
        }
    }

    /// Clears a register's value.
    ///
    /// Just sets the value to [`Option::None`].
    pub fn clear(&mut self) {
        self.value = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_number() {
        let value = Value::from(666);

        let register = Register::new_with_value("X", &value).unwrap();

        assert_eq!(register.read(), Some(value));
    }

    #[test]
    fn test_read_keyword() {
        let value = Value::Keyword(String::from("keyword"));

        let register = Register::new_with_value("X", &value).unwrap();

        assert_eq!(register.read(), Some(value));
    }

    #[test]
    fn test_write_with_number() {
        let mut register = Register::new("X");

        let value = Value::Number(666);

        let expected_register = Register {
            id: String::from("X"),
            value: Some(value.clone()),
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_with_number_too_small_err() {
        let mut register = Register::new_with_value("X", &Value::Number(666)).unwrap();
        let value = Value::Number(-10_000);

        let expected_register = Register {
            id: String::from("X"),
            value: Some(Value::Number(666)),
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert_eq!(result, Err(RegisterWriteError::NumberValueTooSmall(value)));
    }

    #[test]
    fn test_write_with_number_too_large_err() {
        let mut register = Register::new_with_value("X", &Value::Number(666)).unwrap();
        let value = Value::Number(10_000);

        let expected_register = Register {
            id: String::from("X"),
            value: Some(Value::Number(666)),
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert_eq!(result, Err(RegisterWriteError::NumberValueTooLarge(value)));
    }

    #[test]
    fn test_write_with_keyword() {
        let mut register = Register::new("X");
        let value = Value::Keyword(String::from("keyword"));

        let expected_register = Register {
            id: String::from("X"),
            value: Some(value.clone()),
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_with_label_id_err() {
        let mut register = Register::new("X");
        let value = Value::LabelId(String::from("LABEL"));

        let expected_register = Register {
            id: String::from("X"),
            value: None,
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert_eq!(result, Err(RegisterWriteError::WriteWithLabelId(value)));
    }

    #[test]
    fn test_write_with_register_id_err() {
        let mut register = Register::new("X");
        let value = Value::RegisterId(String::from("#NERV"));

        let expected_register = Register {
            id: String::from("X"),
            value: None,
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert_eq!(result, Err(RegisterWriteError::WriteWithRegisterId(value)));
    }

    #[test]
    fn test_clear() {
        let mut register =
            Register::new_with_value("X", &Value::Keyword(String::from("keyword"))).unwrap();

        let expected_register = Register {
            id: String::from("X"),
            value: None,
        };

        register.clear();

        assert_eq!(register, expected_register);
    }
}
