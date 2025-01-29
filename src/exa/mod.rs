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
    WaitingForHostAvailabilityToDropFile,
    WaitingForHostAvailabilityToReplicate,
}

/// Successful responses that occur when an [`Exa`] executes an [`Instruction`].
#[derive(Debug)]
pub enum ExecutionResponse {
    /// Indicates just a normal success.
    Success,
    /// Indicates the [`Exa`] executed an [`Instruction::Link`].
    Link,
    /// Holds a copy of the executing [`Exa`] with a new id.
    Replicate(Exa),
}

impl PartialEq for ExecutionResponse {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ExecutionResponse::Success, ExecutionResponse::Success) => true,
            (ExecutionResponse::Link, ExecutionResponse::Link) => true,
            (ExecutionResponse::Replicate(_), ExecutionResponse::Replicate(_)) => true,
            (ExecutionResponse::Drop(f), ExecutionResponse::Drop(other_f)) => f == other_f,
            _ => false,
        }
    }
}

/// Errors that occur when an [`Exa`] executes an [`Instruction`].
#[derive(Debug, PartialEq, Clone)]
pub enum ExecutionResponseError {
    /// Holds the id of the [`Exa`] that needs to be killed this cycle.
    Halt(String),
    /// Holds the id of the [`Exa`] that needs to be killed next cycle.
    OutOfInstructions(String),
    /// Holds the id of the [`Exa`] that needs to be killed next cycle.
    /// Increase the number of actions, in the metrics, by 1.
    Kill(String),
    /// Holds the numerator and denominator.
    /// The culprit [`Exa`] needs to be killed this cycle.
    DivideByZero(Value, Value),
    /// Holds the operated values.
    /// The culprit [`Exa`] needs to be killed this cycle.
    MathWithKeywords(Value, Value),
    /// Inidicates the [`Exa`] tried to read/write to a file while not holding one.
    /// The culprit [`Exa`] needs to be killed this cycle.
    InvalidFRegisterAccess,
    /// Holds the id of the Hardware Register the [`Exa`] tried to access.
    /// The culprit [`Exa`] needs to be killed this cycle.
    InvalidHardwareRegisterAccess(String),
    /// Holds the id of the [`File`] that the [`Exa`] tried to "GRAB".
    /// The culprit [`Exa`] needs to be killed this cycle.
    InvalidFileAccess(String),
    /// Holds the id of the Link that the [`Exa`] tried to traverse.
    /// The culprit [`Exa`] needs to be killed this cycle.
    InvalidLinkTraversal(String),
}

/// An Exa is a robot that can be controlled by a [`Program`].
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Exa {
    pub id: String,
    x_register: BasicRegister,
    t_register: BasicRegister,
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
            t_register: BasicRegister::new_with_value("T", &Value::Number(0)).unwrap(),
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
            t_register: BasicRegister::new_with_value("T", &Value::Number(0)).unwrap(),
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
    pub fn execute_current_instruction(
        &mut self,
    ) -> Result<ExecutionResponse, ExecutionResponseError> {
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
    use crate::host::link::Link;
    use crate::register::hardware::{AccessMode, HardwareRegister};
    use crate::register::Register;

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
    fn test_execute_current_instruction_failure_out_of_instructions() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        let result = exa.execute_current_instruction();

        assert_eq!(
            result,
            Err(ExecutionResponseError::OutOfInstructions(String::from(
                "XA"
            )))
        );
    }

    #[test]
    fn test_execute_current_instruction_copy() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[String::from("COPY 666 X")]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        let result = exa.execute_current_instruction();

        assert!(result.is_ok());
        assert_eq!(exa.x_register.read(), Ok(Some(Value::Number(666))));
    }

    #[test]
    fn test_execute_current_instruction_copy_to_hardware_register_writeonly() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let hardware_register = HardwareRegister::new("#NERV", AccessMode::WriteOnly);
        let program = Program::new(&[String::from("COPY 666 #NERV")]).unwrap();

        host.borrow_mut()
            .insert_hardware_register(hardware_register);

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        let mut expected_hardware_register = HardwareRegister::new_with_values(
            "#NERV",
            AccessMode::WriteOnly,
            &[Value::Number(666)],
        )
        .unwrap();

        let result = exa.execute_current_instruction();

        assert!(result.is_ok());
        assert_eq!(
            host.borrow_mut().hardware_register_mut("#NERV"),
            Some(&mut expected_hardware_register)
        );
    }

    #[test]
    fn test_execute_current_instruction_copy_from_hardware_register_readonly() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let hardware_register =
            HardwareRegister::new_with_values("#NERV", AccessMode::ReadOnly, &[Value::Number(666)])
                .unwrap();
        let program = Program::new(&[String::from("COPY #NERV T")]).unwrap();

        host.borrow_mut()
            .insert_hardware_register(hardware_register);

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        let result = exa.execute_current_instruction();

        assert!(result.is_ok());
        assert_eq!(exa.t_register.read(), Ok(Some(Value::Number(666))));
        assert!(host
            .borrow_mut()
            .hardware_register_mut("#NERV")
            .unwrap()
            .read()
            .unwrap()
            .is_none());
    }

    #[test]
    fn test_execute_current_instruction_copy_noop_to_hardware_register_readonly() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let hardware_register = HardwareRegister::new("#NERV", AccessMode::ReadOnly);
        let program = Program::new(&[String::from("COPY 666 #NERV")]).unwrap();

        host.borrow_mut()
            .insert_hardware_register(hardware_register);

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        let result = exa.execute_current_instruction();

        assert!(result.is_ok());
        assert!(host
            .borrow_mut()
            .hardware_register_mut("#NERV")
            .unwrap()
            .read()
            .unwrap()
            .is_none());
    }

    #[test]
    fn test_execute_current_instruction_copy_failure_from_hardware_register_writeonly() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let hardware_register = HardwareRegister::new("#NERV", AccessMode::WriteOnly);
        let program = Program::new(&[String::from("COPY #NERV X")]).unwrap();

        host.borrow_mut()
            .insert_hardware_register(hardware_register);

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        let result = exa.execute_current_instruction();

        assert_eq!(
            result,
            Err(ExecutionResponseError::InvalidHardwareRegisterAccess(
                String::from("#NERV")
            ))
        );
    }

    #[test]
    fn test_execute_current_instruction_copy_failure_to_file() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[String::from("COPY X F")]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        let result = exa.execute_current_instruction();

        assert_eq!(result, Err(ExecutionResponseError::InvalidFRegisterAccess));
    }

    #[test]
    fn test_execute_current_instruction_add() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[String::from("ADDI 333 X X")]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        let _ = exa.x_register.write(&Value::Number(222));

        let result = exa.execute_current_instruction();

        assert!(result.is_ok());
        assert_eq!(exa.x_register.read(), Ok(Some(Value::Number(555))));
    }

    #[test]
    fn test_execute_current_instruction_add_failure_math_with_keywords() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[String::from("ADDI 333 X X")]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        let _ = exa.x_register.write(&Value::from("keyword"));

        let result = exa.execute_current_instruction();

        assert_eq!(
            result,
            Err(ExecutionResponseError::MathWithKeywords(
                Value::Number(333),
                Value::from("keyword")
            ))
        );
    }

    #[test]
    fn test_execute_current_instruction_test() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[
            String::from("ADDI 333 X X"),
            String::from("TEST X = 333"),
            String::from("TEST X > 555"),
            String::from("TEST X < 555"),
        ])
        .unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        // ADDI 333 X X
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.x_register.read(), Ok(Some(Value::Number(333))));

        // TEST X = 333
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.t_register.read(), Ok(Some(Value::Number(1))));

        // TEST X > 555
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.t_register.read(), Ok(Some(Value::Number(0))));

        // TEST X < 555
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.t_register.read(), Ok(Some(Value::Number(1))));
    }

    #[test]
    fn test_execute_current_instruction_halt() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));

        host.borrow_mut().insert_exa_id("XA");

        let program = Program::new(&[String::from("HALT")]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        assert!(host.borrow().has_occupying_exa_id("XA"));
        assert_eq!(
            exa.execute_current_instruction(),
            Err(ExecutionResponseError::Halt(String::from("XA")))
        );
        assert!(!host.borrow().has_occupying_exa_id("XA"));
    }

    #[test]
    fn test_execute_current_instruction_link_success() {
        let host_1 = Rc::new(RefCell::new(Host::new("host_1", 9)));
        let host_2 = Rc::new(RefCell::new(Host::new("host_2", 9)));
        let link = Rc::new(RefCell::new(Link::new("800", &host_2, "-1", &host_1)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));

        host_1.borrow_mut().insert_exa_id("XA");
        host_1.borrow_mut().insert_link("800", &link);
        host_2.borrow_mut().insert_link("-1", &link);

        let program = Program::new(&[String::from("LINK 800")]).unwrap();

        let mut exa = Exa::new("XA", program, &host_1, &file_generator);

        let result = exa.execute_current_instruction();

        assert!(host_1.borrow().has_occupying_exa_id("XA"));
        assert_eq!(result, Ok(ExecutionResponse::Link));
        assert!(link.borrow().occupied);
        assert!(!host_1.borrow().has_occupying_exa_id("XA"));
        assert!(host_2.borrow().has_occupying_exa_id("XA"));
    }

    #[test]
    fn test_execute_current_instruction_link_failure_no_link_exists() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[String::from("LINK 800")]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        let result = exa.execute_current_instruction();

        assert_eq!(
            result,
            Err(ExecutionResponseError::InvalidLinkTraversal(String::from(
                "800"
            )))
        );
    }

    #[test]
    fn test_execute_current_instruction_link_failure_waiting_for_link_availability() {
        let host_1 = Rc::new(RefCell::new(Host::new("host_1", 9)));
        let host_2 = Rc::new(RefCell::new(Host::new("host_2", 9)));
        let link = Rc::new(RefCell::new(Link::new("800", &host_2, "-1", &host_1)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));

        link.borrow_mut().occupied = true;
        host_1.borrow_mut().insert_exa_id("XA");
        host_1.borrow_mut().insert_link("800", &link);
        host_2.borrow_mut().insert_link("-1", &link);

        let program = Program::new(&[String::from("LINK 800")]).unwrap();

        let mut exa = Exa::new("XA", program, &host_1, &file_generator);

        let result = exa.execute_current_instruction();

        assert_eq!(result, Ok(ExecutionResponse::Link));
        assert!(link.borrow().occupied);
        assert!(host_1.borrow().has_occupying_exa_id("XA"));
        assert!(!host_2.borrow().has_occupying_exa_id("XA"));
        assert_eq!(exa.state, ExaState::WaitingForLinkToOpen);
    }

    #[test]
    fn test_execute_current_instruction_link_failure_waiting_for_host_availability() {
        let host_1 = Rc::new(RefCell::new(Host::new("host_1", 9)));
        let host_2 = Rc::new(RefCell::new(Host::new("host_2", 0)));
        let link = Rc::new(RefCell::new(Link::new("800", &host_2, "-1", &host_1)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));

        host_1.borrow_mut().insert_exa_id("XA");
        host_1.borrow_mut().insert_link("800", &link);
        host_2.borrow_mut().insert_link("-1", &link);

        let program = Program::new(&[String::from("LINK 800")]).unwrap();

        let mut exa = Exa::new("XA", program, &host_1, &file_generator);

        let result = exa.execute_current_instruction();

        assert_eq!(result, Ok(ExecutionResponse::Link));
        assert!(!link.borrow().occupied);
        assert!(host_1.borrow().has_occupying_exa_id("XA"));
        assert!(!host_2.borrow().has_occupying_exa_id("XA"));
        assert_eq!(exa.state, ExaState::WaitingForLinkToOpen);
    }

    #[test]
    fn test_execute_current_instruction_host() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[String::from("HOST X")]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        let result = exa.execute_current_instruction();

        assert!(result.is_ok());
        assert_eq!(exa.x_register.read(), Ok(Some(Value::from("host"))));
    }

    #[test]
    fn test_execute_current_instruction_grab_success() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let file = File::new_with_contents(
            "200",
            &[
                String::from("keyword1"),
                String::from("666"),
                String::from("keyword2"),
                String::from("333"),
                String::from("keyword3"),
            ],
        );

        host.borrow_mut().insert_file(file.clone());

        let program = Program::new(&[String::from("GRAB 200")]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        assert!(host.borrow().has_file("200"));
        assert!(exa.execute_current_instruction().is_ok());
        assert!(!host.borrow().has_file("200"));
        assert_eq!(exa.file, Some(file));
    }

    #[test]
    fn test_execute_current_instruction_grab_failure_no_file() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[String::from("GRAB 200")]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        let result = exa.execute_current_instruction();

        assert_eq!(
            result,
            Err(ExecutionResponseError::InvalidFileAccess(String::from(
                "200"
            )))
        );
    }

    #[test]
    fn test_execute_current_instruction_grab_failure_waiting() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let file = File::new_with_contents(
            "200",
            &[
                String::from("keyword1"),
                String::from("666"),
                String::from("keyword2"),
                String::from("333"),
                String::from("keyword3"),
            ],
        );

        let _ = host.borrow_mut().insert_pending_file(file.clone());

        let program = Program::new(&[String::from("GRAB 200")]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        let result = exa.execute_current_instruction();

        assert!(result.is_ok());
        assert!(host.borrow().has_file("200"));
        assert!(exa.file.is_none());
        assert_eq!(exa.state, ExaState::WaitingForFile);
    }

    #[test]
    fn test_execute_current_instruction_void_f() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let file = File::new_with_contents(
            "200",
            &[
                String::from("keyword1"),
                String::from("666"),
                String::from("keyword2"),
                String::from("333"),
                String::from("keyword3"),
            ],
        );

        host.borrow_mut().insert_file(file.clone());

        let program = Program::new(&[String::from("GRAB 200"), String::from("VOID F")]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        let expected_file_after_void = File::new_with_contents(
            "200",
            &[
                String::from("666"),
                String::from("keyword2"),
                String::from("333"),
                String::from("keyword3"),
            ],
        );

        // GRAB 200
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.file, Some(file));

        // VOID F
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.file, Some(expected_file_after_void));
    }

    #[test]
    fn test_execute_current_instruction_seek() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let file = File::new_with_contents(
            "200",
            &[
                String::from("keyword1"),
                String::from("666"),
                String::from("keyword2"),
                String::from("333"),
                String::from("keyword3"),
            ],
        );

        host.borrow_mut().insert_file(file.clone());

        let program = Program::new(&[
            String::from("GRAB 200"),
            String::from("SEEK 2"),
            String::from("COPY F X"),
        ])
        .unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        // GRAB 200
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.file, Some(file));

        // SEEK 2
        assert!(exa.execute_current_instruction().is_ok());

        // COPY F X
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.x_register.read(), Ok(Some(Value::from("keyword2"))));
    }

    #[test]
    fn test_execute_current_instruction_testeof() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let file = File::new_with_contents(
            "200",
            &[
                String::from("keyword1"),
                String::from("666"),
                String::from("keyword2"),
                String::from("333"),
                String::from("keyword3"),
            ],
        );

        host.borrow_mut().insert_file(file.clone());

        let program = Program::new(&[
            String::from("GRAB 200"),
            String::from("TEST EOF"),
            String::from("SEEK 9999"),
            String::from("TEST EOF"),
        ])
        .unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        // GRAB 200
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.file, Some(file));

        // TEST EOF
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.t_register.read(), Ok(Some(Value::Number(0))));

        // SEEK 9999
        assert!(exa.execute_current_instruction().is_ok());

        // TEST EOF
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.t_register.read(), Ok(Some(Value::Number(1))));
    }

    #[test]
    fn test_execute_current_instruction_make() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[String::from("MAKE")]).unwrap();

        let mut exa_1 = Exa::new("XA", program.clone(), &host, &file_generator);
        let mut exa_2 = Exa::new("XB", program, &host, &file_generator);

        let expected_file_1 = File::new("400");
        let expected_file_2 = File::new("401");

        assert!(exa_1.file.is_none());
        assert!(exa_2.file.is_none());

        assert!(exa_1.execute_current_instruction().is_ok());
        assert!(exa_2.execute_current_instruction().is_ok());

        assert_eq!(exa_1.file, Some(expected_file_1));
        assert_eq!(exa_2.file, Some(expected_file_2));
    }

    #[test]
    fn test_execute_current_instruction_file() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[String::from("MAKE"), String::from("FILE X")]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        // MAKE
        assert!(exa.execute_current_instruction().is_ok());

        // FILE X
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(
            exa.x_register.read(),
            Ok(Some(Value::Keyword(String::from("400"))))
        );
    }

    #[test]
    fn test_execute_current_instruction_drop_success() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[String::from("MAKE"), String::from("DROP")]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        // MAKE
        assert!(exa.execute_current_instruction().is_ok());

        // DROP
        assert!(!host.borrow().has_file("400"));
        assert!(exa.file.is_some());
        assert!(exa.execute_current_instruction().is_ok());
        assert!(host.borrow().has_file("400"));
        assert!(exa.file.is_none());
    }

    #[test]
    fn test_execute_current_instruction_drop_waiting() {
        let host = Rc::new(RefCell::new(Host::new("host", 1)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));

        host.borrow_mut().insert_exa_id("XA");

        let program = Program::new(&[String::from("MAKE"), String::from("DROP")]).unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        // MAKE
        assert!(exa.execute_current_instruction().is_ok());

        // DROP
        assert!(!host.borrow().has_file("400"));
        assert!(exa.file.is_some());
        assert!(exa.execute_current_instruction().is_ok());
        assert!(!host.borrow().has_file("400"));
        assert!(exa.file.is_some());
        assert_eq!(exa.state, ExaState::WaitingForHostAvailabilityToDropFile);
    }

    #[test]
    fn test_execute_current_instruction_jump() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[
            String::from("ADDI 300 X X"),
            String::from("JUMP LABEL"),
            String::from("HALT"),
            String::from("MARK LABEL"),
            String::from("MULI 2 X X"),
        ])
        .unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        // ADDI 300 X X
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.x_register.read(), Ok(Some(Value::Number(300))));

        // JUMP LABEL
        assert!(exa.execute_current_instruction().is_ok());

        // MULI 2 X X
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.x_register.read(), Ok(Some(Value::Number(600))));
    }

    #[test]
    fn test_execute_current_instruction_jump_if_true() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[
            String::from("ADDI 300 X X"),
            String::from("TEST X = 300"),
            String::from("TJMP LABEL"),
            String::from("HALT"),
            String::from("MARK LABEL"),
            String::from("MULI 2 X X"),
        ])
        .unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        // ADDI 300 X X
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.x_register.read(), Ok(Some(Value::Number(300))));

        // TEST X = 300
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.t_register.read(), Ok(Some(Value::Number(1))));

        // TJMP LABEL
        assert!(exa.execute_current_instruction().is_ok());

        // MULI 2 X X
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.x_register.read(), Ok(Some(Value::Number(600))));
    }

    #[test]
    fn test_execute_current_instruction_jump_if_false() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));
        let program = Program::new(&[
            String::from("ADDI 300 X X"),
            String::from("TEST X = 300"),
            String::from("FJMP LABEL"),
            String::from("HALT"),
            String::from("MARK LABEL"),
            String::from("MULI 2 X X"),
        ])
        .unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        // ADDI 300 X X
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.x_register.read(), Ok(Some(Value::Number(300))));

        // TEST X = 300
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.t_register.read(), Ok(Some(Value::Number(1))));

        // FJMP LABEL
        assert!(exa.execute_current_instruction().is_ok());

        // HALT
        assert_eq!(
            exa.execute_current_instruction(),
            Err(ExecutionResponseError::Halt(String::from("XA")))
        );
    }

    #[test]
    fn test_execute_current_instruction_replicate() {
        let host = Rc::new(RefCell::new(Host::new("host", 9)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));

        host.borrow_mut().insert_exa_id("XA");

        let program = Program::new(&[
            String::from("COPY 333 X"),
            String::from("MAKE"),
            String::from("REPL LABEL"),
            String::from("HALT"),
            String::from("MARK LABEL"),
            String::from("MULI 2 X X"),
        ])
        .unwrap();

        let mut replicated_exa: Option<Exa> = None;
        let mut exa = Exa::new("XA", program, &host, &file_generator);

        // XA - COPY 333 X
        assert!(exa.execute_current_instruction().is_ok());
        assert_eq!(exa.x_register.read(), Ok(Some(Value::Number(333))));

        // XA - MAKE
        assert!(exa.file.is_none());
        assert!(exa.execute_current_instruction().is_ok());
        assert!(exa.file.is_some());

        // XA - REPL LABEL
        assert!(host.borrow().has_occupying_exa_id("XA"));
        assert!(!host.borrow().has_occupying_exa_id("XA:0"));

        if let Ok(ExecutionResponse::Replicate(result)) = exa.execute_current_instruction() {
            replicated_exa = Some(result);
        }

        assert!(replicated_exa.is_some());
        assert!(host.borrow().has_occupying_exa_id("XA"));
        assert!(host.borrow().has_occupying_exa_id("XA:0"));

        // XA - HALT
        assert!(host.borrow().has_occupying_exa_id("XA"));
        assert_eq!(
            exa.execute_current_instruction(),
            Err(ExecutionResponseError::Halt(String::from("XA"))),
        );
        assert!(!host.borrow().has_occupying_exa_id("XA"));

        // XA:0 - MULI 2 X X
        assert!(replicated_exa
            .as_mut()
            .unwrap()
            .execute_current_instruction()
            .is_ok());
        assert_eq!(
            replicated_exa.unwrap().x_register.read(),
            Ok(Some(Value::Number(666)))
        );
    }

    #[test]
    fn test_execute_current_instruction_replicate_waiting() {
        let host = Rc::new(RefCell::new(Host::new("host", 1)));
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));
        let file_generator = Rc::new(RefCell::new(Generator::new(&id_generator)));

        host.borrow_mut().insert_exa_id("XA");

        let program = Program::new(&[
            String::from("REPL LABEL"),
            String::from("HALT"),
            String::from("MARK LABEL"),
            String::from("MULI 2 X X"),
        ])
        .unwrap();

        let mut exa = Exa::new("XA", program, &host, &file_generator);

        // XA - REPL LABEL
        assert!(host.borrow().has_occupying_exa_id("XA"));
        assert!(!host.borrow().has_occupying_exa_id("XA:0"));
        assert_eq!(
            exa.execute_current_instruction(),
            Ok(ExecutionResponse::Success)
        );
        assert!(host.borrow().has_occupying_exa_id("XA"));
        assert!(!host.borrow().has_occupying_exa_id("XA:0"));
        assert_eq!(exa.state, ExaState::WaitingForHostAvailabilityToReplicate);
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
    fn test_execute_current_instruction_void_m() {
        unimplemented!()
    }

    #[test]
    fn test_execute_current_instruction_test_mrd() {
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
