pub mod exa;
pub mod instruction;
pub mod value;

use crate::value::Value;
use crate::instruction::Instruction;

fn main() {
    let lhs = Value::Number(-127);
    let rhs = Value::Number(128);
    let destination = Value::RegisterId("X".to_string());
    let add_instruction = Instruction::Add(lhs.clone(), rhs, destination);

    println!("Hello, world!");
    println!("Here is my number: {lhs:?}");
    println!("Here is my add instruction: {add_instruction:?}");
}
