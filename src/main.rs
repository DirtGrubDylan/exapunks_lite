pub mod exa;
pub mod program;
pub mod register;
pub mod util;
pub mod value;

use crate::program::instruction::Instruction;
use value::Value;

fn main() {
    let lhs = Value::Number(-127);
    let rhs = Value::Number(128);
    let destination = Value::RegisterId("X".to_string());
    let add_instruction = Instruction::Add(lhs.clone(), rhs, destination);

    println!("Hello, world!");
    println!("Here is my number: {lhs:?}");
    println!("Here is my add instruction: {add_instruction:?}");
}
