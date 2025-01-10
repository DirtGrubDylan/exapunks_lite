use std::collections::VecDeque;

use crate::value::Value;

use super::{AccessError, Register};

/// The access mode dictates if an Exa can read or write from a hardware register.
#[derive(Debug, PartialEq, Clone)]
pub enum AccessMode {
    ReadOnly,
    WriteOnly,
}

/// A Hardware Register holds predefined queue of [`Value`]s and an [`AccessMode`].
///
/// The register can pop an item from the front of the queue or append an item to the back.
#[derive(Debug, PartialEq, Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct HardwareRegister {
    id: String,
    values: VecDeque<Value>,
    mode: AccessMode,
}

impl HardwareRegister {
    /// Returns a register with a given id, access mode, and empty queue of [`Value`]s.
    #[must_use]
    pub fn new(id: &str, mode: AccessMode) -> Self {
        HardwareRegister {
            id: id.to_string(),
            values: VecDeque::new(),
            mode,
        }
    }

    /// Returns a register with a given id, access mode, and queue of [`Value`]s.
    ///
    /// # Errors
    ///
    /// * `NumberValueTooSmall` - if given value is a number less than -9999.
    /// * `NumberValueTooLarge` - if given value is a number greater than 9999.
    /// * `WriteWithLabelId` - if given value is a [`Value::LabelId`].
    /// * `WriteWithRegisterId` - if given value is a [`Value::RegisterId`].
    pub fn new_with_values(
        id: &str,
        mode: AccessMode,
        values: &[Value],
    ) -> Result<Self, AccessError> {
        // Temporarily allow writing, which is important for instantiating read-only registers.
        let mut register = Self::new(id, AccessMode::WriteOnly);

        for value in values {
            register.write(value)?;
        }

        register.mode = mode;

        Ok(register)
    }
}

impl Register for HardwareRegister {
    /// Returns the possible [`Value`] from the front of the register's queue;
    ///
    /// # Errors
    ///
    /// * `InvalidReadAccess` - if the register can only be written to.
    fn read(&self) -> Result<Option<Value>, AccessError> {
        if self.mode == AccessMode::WriteOnly {
            Err(AccessError::InvalidReadAccess)
        } else {
            Ok(self.values.front().cloned())
        }
    }

    /// Pops the front of the register's queue, and returns the possible [`Value`].
    ///
    /// # Errors
    ///
    /// * `InvalidReadAccess` - if the register can only be written to.
    fn read_mut(&mut self) -> Result<Option<Value>, AccessError> {
        if self.mode == AccessMode::WriteOnly {
            Err(AccessError::InvalidReadAccess)
        } else {
            Ok(self.values.pop_front().clone())
        }
    }

    /// Appends a given [`Value`] to the register's queue.
    ///
    /// If there is an error, or the register is read-only, the register will be unchanged.
    ///
    /// NOTE: Read-only registers allow writing, but it's a no-op.
    ///
    /// # Errors
    ///
    /// * `NumberValueTooSmall` - if given value is a number less than -9999.
    /// * `NumberValueTooLarge` - if given value is a number greater than 9999.
    /// * `WriteWithLabelId` - if given value is a [`Value::LabelId`].
    /// * `WriteWithRegisterId` - if given value is a [`Value::RegisterId`].
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
                if self.mode == AccessMode::WriteOnly {
                    self.values.push_back(value.clone());
                }

                Ok(())
            }
        }
    }

    /// Empties a register's queue.
    fn clear(&mut self) {
        self.values = VecDeque::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_number() {
        let values = [Value::from(666), Value::from(333)];

        let register =
            HardwareRegister::new_with_values("X", AccessMode::ReadOnly, &values).unwrap();

        assert_eq!(register.read(), Ok(Some(Value::from(666))));
        assert_eq!(register.values, VecDeque::from(values));
    }

    #[test]
    fn test_read_number_write_only_err() {
        let values = [Value::from(666), Value::from(333)];

        let register =
            HardwareRegister::new_with_values("X", AccessMode::WriteOnly, &values).unwrap();

        assert_eq!(register.read(), Err(AccessError::InvalidReadAccess));
        assert_eq!(register.values, VecDeque::from(values));
    }

    #[test]
    fn test_read_keyword() {
        let values = [Value::Keyword(String::from("keyword")), Value::from(666)];

        let register =
            HardwareRegister::new_with_values("X", AccessMode::ReadOnly, &values).unwrap();

        assert_eq!(
            register.read(),
            Ok(Some(Value::Keyword(String::from("keyword"))))
        );
        assert_eq!(register.values, VecDeque::from(values));
    }

    #[test]
    fn test_read_mut_number() {
        let values = [Value::from(666), Value::from(333)];

        let mut register =
            HardwareRegister::new_with_values("X", AccessMode::ReadOnly, &values).unwrap();

        assert_eq!(register.read_mut(), Ok(Some(Value::from(666))));
        assert_eq!(register.values, VecDeque::from([Value::from(333)]));
    }

    #[test]
    fn test_read_mut_number_write_only_err() {
        let values = [Value::from(666), Value::from(333)];

        let mut register =
            HardwareRegister::new_with_values("X", AccessMode::WriteOnly, &values).unwrap();

        assert_eq!(register.read_mut(), Err(AccessError::InvalidReadAccess));
        assert_eq!(register.values, VecDeque::from(values));
    }

    #[test]
    fn test_read_mut_keyword() {
        let values = [Value::Keyword(String::from("keyword")), Value::from(666)];

        let mut register =
            HardwareRegister::new_with_values("X", AccessMode::ReadOnly, &values).unwrap();

        assert_eq!(
            register.read_mut(),
            Ok(Some(Value::Keyword(String::from("keyword"))))
        );
        assert_eq!(register.values, VecDeque::from([Value::from(666)]));
    }

    #[test]
    fn test_write_with_number() {
        let mut register = HardwareRegister::new("X", AccessMode::WriteOnly);

        let value = Value::Number(666);

        let expected_register = HardwareRegister {
            id: String::from("X"),
            values: VecDeque::from([value.clone()]),
            mode: AccessMode::WriteOnly,
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_with_number_too_small_err() {
        let mut register = HardwareRegister::new("X", AccessMode::WriteOnly);
        let value = Value::Number(-10_000);

        let expected_register = HardwareRegister {
            id: String::from("X"),
            values: VecDeque::new(),
            mode: AccessMode::WriteOnly,
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert_eq!(result, Err(AccessError::NumberValueTooSmall(value)));
    }

    #[test]
    fn test_write_with_number_too_large_err() {
        let mut register = HardwareRegister::new("X", AccessMode::WriteOnly);
        let value = Value::Number(10_000);

        let expected_register = HardwareRegister {
            id: String::from("X"),
            values: VecDeque::new(),
            mode: AccessMode::WriteOnly,
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert_eq!(result, Err(AccessError::NumberValueTooLarge(value)));
    }

    #[test]
    fn test_write_with_number_read_only_noop() {
        let mut register = HardwareRegister::new("X", AccessMode::ReadOnly);
        let value = Value::Number(666);

        let expected_register = HardwareRegister {
            id: String::from("X"),
            values: VecDeque::new(),
            mode: AccessMode::ReadOnly,
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_with_keyword() {
        let mut register = HardwareRegister::new("X", AccessMode::WriteOnly);
        let value = Value::Keyword(String::from("keyword"));

        let expected_register = HardwareRegister {
            id: String::from("X"),
            values: VecDeque::from([value.clone()]),
            mode: AccessMode::WriteOnly,
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert!(result.is_ok());
    }

    #[test]
    fn test_write_with_label_id_err() {
        let mut register = HardwareRegister::new("X", AccessMode::WriteOnly);
        let value = Value::LabelId(String::from("LABEL"));

        let expected_register = HardwareRegister {
            id: String::from("X"),
            values: VecDeque::new(),
            mode: AccessMode::WriteOnly,
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert_eq!(result, Err(AccessError::WriteWithLabelId(value)));
    }

    #[test]
    fn test_write_with_register_id_err() {
        let mut register = HardwareRegister::new("X", AccessMode::WriteOnly);
        let value = Value::RegisterId(String::from("#NERV"));

        let expected_register = HardwareRegister {
            id: String::from("X"),
            values: VecDeque::new(),
            mode: AccessMode::WriteOnly,
        };

        let result = register.write(&value);

        assert_eq!(register, expected_register);
        assert_eq!(result, Err(AccessError::WriteWithRegisterId(value)));
    }

    #[test]
    fn test_clear() {
        let values = [Value::from(666), Value::from(333)];

        let mut register =
            HardwareRegister::new_with_values("X", AccessMode::WriteOnly, &values).unwrap();

        let expected_register = HardwareRegister {
            id: String::from("X"),
            values: VecDeque::new(),
            mode: AccessMode::WriteOnly,
        };

        register.clear();

        assert_eq!(register, expected_register);
    }
}
