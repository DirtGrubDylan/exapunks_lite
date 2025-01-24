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
    /// This shouldn't panic, but [`BasicRegister`]s are created and unwrapped.
    pub fn new(
        id: &str,
        program: Program,
        host: &Rc<RefCell<Host>>,
        file_generator: &Rc<RefCell<Generator>>,
    ) -> Self {
        Exa {
            id: id.to_string(),
            x_register: BasicRegister::new_with_value("X", &Value::Number(0)).unwrap(),
            y_register: BasicRegister::new_with_value("Y", &Value::Number(0)).unwrap(),
            f_register: BasicRegister::new("F"),
            host: Rc::downgrade(host),
            program,
            file: None,
            file_generator: Rc::downgrade(file_generator),
            next_exa_id: 0,
            communication_mode: CommunicationMode::Global,
            state: ExaState::Running,
        }
    }

    /// Creates a new Exa with a given id, [`Host`], file [`Generator`], and file path.
    ///
    /// # Panics
    ///
    /// If the file path does not exist **OR** does not contain a valid [`Program`].
    pub fn new_from_file(
        id: &str,
        program_file_path: &str,
        host: &Rc<RefCell<Host>>,
        file_generator: &Rc<RefCell<Generator>>,
    ) -> Self {
        let program = Program::new_from_file(program_file_path).unwrap();

        Exa {
            id: id.to_string(),
            x_register: BasicRegister::new_with_value("X", &Value::Number(0)).unwrap(),
            y_register: BasicRegister::new_with_value("Y", &Value::Number(0)).unwrap(),
            f_register: BasicRegister::new("F"),
            host: Rc::downgrade(host),
            program,
            file: None,
            file_generator: Rc::downgrade(file_generator),
            next_exa_id: 0,
            communication_mode: CommunicationMode::Global,
            state: ExaState::Running,
        }
    }

    /// Returns the current [`Instruction`] and its index, if possible.
    ///
    /// This will not increase the [`Program`] stack.
    pub fn peak_current_instruction(&self) -> Option<(usize, Instruction)> {
        self.program.peak_current_instruction()
    }

    /// Executes the current [`Instruction`] and returns nothing or the [`ExecutionResponseError`].
    ///
    /// This will increase the [`Program`] stack by 1.
    ///
    /// This method will call any of the various private methods to execute the current
    /// [`Instruction`] on the [`Program`] stack.
    ///
    /// # Errors
    ///
    /// See [`ExecutionResponseError`].
    pub fn execute_current_instruction(&mut self) -> Result<(), ExecutionResponseError> {
        unimplemented!()
    }

    /// Returns the next id for the replicated Exa.
    pub fn next_replicated_exa_id(&mut self) -> String {
        let result = self.id.clone() + ":" + &self.next_exa_id.to_string();

        self.next_exa_id += 1;

        result
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
    fn test_peak_current_instruction() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));

        let mut exa = Exa::new_from_file(
            "XA",
            "test_files/simple_program.exa",
            &host,
            &file_generator,
        );

        let expected = vec![
            (0, Instruction::Link(Value::Number(800))),
            (
                2,
                Instruction::Copy(Value::Number(4), Value::RegisterId(String::from("X"))),
            ),
            (
                6,
                Instruction::Subtract(
                    Value::RegisterId(String::from("X")),
                    Value::Number(1),
                    Value::RegisterId(String::from("X")),
                ),
            ),
            (
                7,
                Instruction::TestEqual(Value::RegisterId(String::from("X")), Value::Number(0)),
            ),
            (
                8,
                Instruction::JumpIfFalse(Value::LabelId(String::from("THIS_LABEL"))),
            ),
            (10, Instruction::Halt),
        ];

        let mut results = Vec::new();

        while let Some(instruction) = exa.peak_current_instruction() {
            results.push(instruction);

            exa.program.get_current_instruction();
        }

        assert!(exa.peak_current_instruction().is_none());
        assert_eq!(results, expected);
    }

    #[test]
    fn test_execute_current_instruction_copy() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_copy_noop_hardware_register_readonly() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_copy_failure_hardware_register_writeonly() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_add() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_add_failure() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_test() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_halt() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_link_success() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_link_failure_no_link_exists() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_link_failure_waiting_for_link_availability() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_link_failure_waiting_for_host_availability() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_host() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_void_f() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_void_m() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_testeof() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_jump() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_replicate() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_kill() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_mode() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_test_mrd() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_make() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_grab_success() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_grab_failure_no_file() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_grab_failure_waiting() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_file() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_seek() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_drop_success() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_drop_waiting() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_rand() {
        unimplemented!()
    }

    #[test]
    fn test_next_replicated_exa_id() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));

        let mut exa = Exa::new_from_file(
            "XA:0",
            "test_files/simple_program.exa",
            &host,
            &file_generator,
        );

        let expected_1 = String::from("XA:0:0");
        let expected_2 = String::from("XA:0:1");

        let result_1 = exa.next_replicated_exa_id();
        let result_2 = exa.next_replicated_exa_id();

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
    }
}
