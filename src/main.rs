pub mod value;

fn main() {
    let number = value::Value::Number(-127);

    println!("Hello, world!");
    println!("Here is my number: {number:?}");
}
