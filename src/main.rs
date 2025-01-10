pub mod exa;
pub mod program;
pub mod register;
pub mod util;
pub mod value;

use crate::program::instruction::Instruction;
use crate::program::Program;
use crate::register::basic::BasicRegister;
use crate::register::hardware::{AccessMode, HardwareRegister};
use crate::register::Register;
use value::Value;

fn main() {
    let lhs = Value::Number(-127);
    let rhs = Value::Number(128);
    let destination = Value::RegisterId("X".to_string());
    let add_instruction = Instruction::Add(lhs.clone(), rhs.clone(), destination);
    let program = Program::new_from_file("test_files/simple_program.exa").unwrap();
    let register = BasicRegister::new_with_value("X", &lhs).unwrap();
    let mut hardware_register =
        HardwareRegister::new_with_values("X", AccessMode::ReadOnly, &[lhs.clone(), rhs.clone()])
            .unwrap();

    println!("Here is my number: {lhs:?}");
    println!("Here is my add instruction: {add_instruction:?}");
    println!("Here is my register: {register:?}");
    println!("Here is my register read: {:?}", register.read());
    println!("Here is my hardware_register: {hardware_register:?}");
    println!(
        "Here is my hardware_register read: {:?}",
        hardware_register.read_mut()
    );
    println!("Here is my hardware_register after read: {hardware_register:?}");
    println!("Here is my simple program:\n{program}");
}
