pub mod basic;
// pub mod hardware;

use crate::value::Value;

/// A dummy struct to hold possible register access errors.
#[derive(Debug, PartialEq, Clone)]
pub enum AccessError {
    NumberValueTooSmall(Value),
    NumberValueTooLarge(Value),
    WriteWithLabelId(Value),
    WriteWithRegisterId(Value),
    InvalidReadAccess,
}

/// A trait that all registers share for reading, writing, and clearing the contents.
pub trait Register {
    /// Returns a [`Value`] from the register.
    #[must_use]
    fn read(&self) -> Option<Value>;

    /// Write a given [`Value`] to the register.
    ///
    /// If there is an error, the register will be unchanged.
    ///
    /// # Errors
    ///
    /// * `NumberValueTooSmall` - if given value is a number less than -9999.
    /// * `NumberValueTooLarge` - if given value is a number greater than 9999.
    /// * `WriteWithLabelId` - if given value is a [`Value::LabelId`].
    /// * `WriteWithRegisterId` - if given value is a [`Value::RegisterId`].
    /// * `InvalidReadAccess` - if the register can only be written to.
    fn write(&mut self, value: &Value) -> Result<(), AccessError>;

    /// Clears a register's state, except it's id.
    fn clear(&mut self);
}
