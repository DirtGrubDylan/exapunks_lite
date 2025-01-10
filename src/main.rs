pub mod exa;
pub mod program;
pub mod register;
pub mod util;
pub mod value;

use crate::program::instruction::Instruction;
use crate::program::Program;
use crate::register::basic::BasicRegister;
use crate::register::Register;
use value::Value;

fn main() {
    let lhs = Value::Number(-127);
    let rhs = Value::Number(128);
    let destination = Value::RegisterId("X".to_string());
    let add_instruction = Instruction::Add(lhs.clone(), rhs, destination);
    let program = Program::new_from_file("test_files/simple_program.exa").unwrap();
    let register = BasicRegister::new_with_value("X", &lhs).unwrap();

    println!("Here is my number: {lhs:?}");
    println!("Here is my add instruction: {add_instruction:?}");
    println!("Here is my register: {register:?}");
    println!("Here is my register read: {:?}", register.read());
    println!("Here is my simple program:\n{program}");
}
