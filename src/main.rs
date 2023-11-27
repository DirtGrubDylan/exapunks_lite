pub mod value;

use value::Value;

fn main() {
    let number = Value::Number(-127);

    println!("Hello, world!");
    println!("Here is my number: {number:?}");
}
