use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::file::generator::Generator;
use crate::file::File;
use crate::host::Host;
use crate::program::instruction::Instruction;
use crate::program::Program;
use crate::register::basic::BasicRegister;
use crate::value::Value;

/// This enum dictates which communication mode the [`Exa`] is in.
///
/// * Global - The "M" register can be written/read by all other EXAs also in Global mode.
/// * Local - The "M" register can be written/read by all other EXAs in the same [`Host`] that are
///   also in Local mode.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CommunicationMode {
    Global,
    Local,
}

/// Indicates what state the [`Exa`] is in.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ExaState {
    Running,
    WaitingForFile,
    WaitingForMRead,
    WaitingForMWrite,
    WaitingForLinkToOpen,
}

/// Errors that occur when an [`Exa`] executes an [`Instruction`].
#[derive(Debug)]
pub enum ExecutionResponseError {
    /// Holds the id of the [`Exa`] that needs to be killed this cycle.
    Halt(String),
    /// Increase the number of actions, in the metrics, by 1.
    Link,
    /// Holds the id of the [`Exa`] that needs to be killed next cycle.
    OutOfInstructions(String),
    /// Holds the id of the [`Exa`] that needs to be killed next cycle.
    /// Increase the number of actions, in the metrics, by 1.
    Kill(String),
    /// Holds the [`Exa`] that should be added to the list of active [`Exa`]s.
    Repl(Exa),
    /// Holds the numerator and denominator.
    /// The culprit [`Exa`] needs to be killed this cycle.
    DivideByZero(Value, Value),
    /// Holds the operated values.
    /// The culprit [`Exa`] needs to be killed this cycle.
    MathWithKeywords(Value, Value),
    /// The culprit [`Exa`] needs to be killed this cycle.
    InvalidFRegisterAccess,
    /// Holds the id of the Hardware Register the [`Exa`] tried to access.
    /// The culprit [`Exa`] needs to be killed this cycle.
    InvalidHardwareRegisterAccess,
    /// Holds the id of the [`File`] that the [`Exa`] tried to "GRAB".
    /// The culprit [`Exa`] needs to be killed this cycle.
    InvalidFileAccess(Value),
    /// Holds the id of the Link that the [`Exa`] tried to traverse.
    /// The culprit [`Exa`] needs to be killed this cycle.
    InvalidLinkTraversal(Value),
}

/// An Exa is a robot that can be controlled by a [`Program`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Exa {
    pub id: String,
    x_register: BasicRegister,
    y_register: BasicRegister,
    f_register: BasicRegister,
    host: Weak<RefCell<Host>>,
    program: Program,
    file: Option<File>,
    file_generator: Weak<RefCell<Generator>>,
    next_exa_id: usize,
    communication_mode: CommunicationMode,
    state: ExaState,
}

impl Exa {
    /// Creates a new Exa with a given id, [`Host`], file [`Generator`], and [`Program`].
    ///
    /// # Panics
    ///
    /// This shouldn't panic, but I am creating [`BasicRegister`]s and unwrapping them.
    pub fn new(
        id: &str,
        program: Program,
        host: Rc<RefCell<Host>>,
        file_generator: Rc<RefCell<Generator>>,
    ) -> Self {
        Exa {
            id: id.to_string(),
            x_register: BasicRegister::new_with_value("X", &Value::Number(0)).unwrap(),
            y_register: BasicRegister::new_with_value("Y", &Value::Number(0)).unwrap(),
            f_register: BasicRegister::new_with_value("F", &Value::Number(0)).unwrap(),
            host: Rc::downgrade(&host),
            program,
            file: None,
            file_generator: Rc::downgrade(&file_generator),
            next_exa_id: 0,
            communication_mode: CommunicationMode::Global,
            state: ExaState::Running,
        }
    }

    /// Returns the next [`Instruction`] and its index, if possible.
    pub fn get_next_instruction(&self) -> Option<(usize, Instruction)> {
        unimplemented!()
    }

    /// Executes the next [`Instruction`] and returns nothing or the [`ExecutionResponseError`].
    ///
    /// This method will call any of the various private methods to execute the next [`Instruction`]
    /// on the [`Program`] stack.
    ///
    /// # Errors
    ///
    /// See [`ExecutionResponseError`].
    pub fn execute_next_instruction(&mut self) -> Result<(), ExecutionResponseError> {
        unimplemented!()
    }

    /// Returns the next id for the replicated Exa.
    pub fn next_replicated_exa_id(&mut self) -> String {
        unimplemented!()
    }

    /// Takes the [`File`] the Exa is holding, if possible.
    ///
    /// This passes ownership of the [`File`] to the caller and sets the Exa's file to
    /// [`Option::None`].
    pub fn drop_file(&mut self) -> Option<File> {
        self.file.take()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::file::id_generator::IdGenerator;

    #[test]
    fn test_get_next_instruction() {
        let program = Program::new_from_file("test_files/simple_program.exa").unwrap();
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));

        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_copy() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_copy_noop_hardware_register_readonly() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_copy_failure_hardware_register_writeonly() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_add() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_add_failure() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_test() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_halt() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_link_success() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_link_failure_no_link_exists() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_link_failure_waiting_for_link_availability() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_link_failure_waiting_for_host_availability() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_host() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_void_f() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_void_m() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_testeof() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_jump() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_replicate() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_kill() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_mode() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_test_mrd() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_make() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_grab_success() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_grab_failure_no_file() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_grab_failure_waiting() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_file() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_seek() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_drop_success() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_drop_waiting() {
        unimplemented!()
    }

    #[test]
    fn test_execute_next_instruction_rand() {
        unimplemented!()
    }

    #[test]
    fn test_next_replicated_exa_id() {
        unimplemented!()
    }
}
