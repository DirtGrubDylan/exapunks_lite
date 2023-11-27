use std::str::FromStr;

use crate::value::Value;

/// An instruction describes a command for an [`Exa`] to execute.
///
/// Instructions are comprised of [`Value`]s which tell the [`Exa`] how to extract the information
/// to execute.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
    Copy(Value, Value),
    Add(Value, Value, Value),
    Subtract(Value, Value, Value),
    Multiply(Value, Value, Value),
    Divide(Value, Value, Value),
    Modulo(Value, Value, Value),
    Swiz(Value, Value, Value),
    Mark(Value),
    Jump(Value),
    JumpIfTrue(Value),
    JumpIfFalse(Value),
    TestEqual(Value, Value),
    TestGreaterThan(Value, Value),
    TestLessThan(Value, Value),
    Replicate(Value),
    Halt,
    Kill,
    Link(Value),
    Host(Value),
    Mode,
    VoidM,
    TestMRD,
    Make,
    Grab(Value),
    File(Value),
    Seek(Value),
    VoidF,
    Drop,
    Wipe,
    TestEndOfFile,
    Note,
    NoOp,
    Random(Value, Value, Value),
}

/// A dummy struct to indicate that there was an error on the [`FromStr`] implementation.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParseError(String);

impl Instruction {
    fn parse_copy(line: &str) -> Result<Self, ParseError> {
        let error = Err(ParseError(line.to_string()));
        let split_line: Vec<&str> = line.split(' ').collect();

        if split_line.len() != 3 {
            return error;
        }

        let source_result = Value::new_number_or_register_id(split_line[1]);
        let destination_result = Value::new_register_id(split_line[2]);

        match (source_result, destination_result) {
            (Ok(source), Ok(destination)) => Ok(Self::Copy(source, destination)),
            _ => error,
        }
    }

    fn parse_add(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_subtract(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_multiply(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_divide(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_modulo(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_swiz(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_mark(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_jump(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_jump_if_true(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_jump_if_false(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_test_equal(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_test_greater_than(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_test_less_than(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_replicate(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_halt(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_kill(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_link(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_host(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_mode(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_voidm(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_testmrd(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_make(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_grab(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_file(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_seek(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_voidf(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_drop(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_wipe(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_test_end_of_file(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_note(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_noop(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }

    fn parse_random(line: &str) -> Result<Self, ParseError> {
        unimplemented!()
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let error = Err(ParseError(input.to_string()));

        match input {
            _ if input.is_empty() => error,
            _ if input.starts_with("COPY") => Self::parse_copy(input),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Instruction, ParseError, Value};

    #[test]
    fn test_parse_empty() {
        let empty_instruction = "";

        let expected_err: Result<Instruction, ParseError> = Err(ParseError(String::new()));

        let err = empty_instruction.parse();

        assert_eq!(err, expected_err);
    }

    #[test]
    fn test_parse_copy() {
        let instruction1 = "COPY -9999 X";
        let instruction2 = "COPY T X";
        let instruction3 = "COPY 666 #NERV";
        let instruction4 = "COPY #NERV X";
        let invalid_instruction1 = "COPY #NERV 6666";
        let invalid_instruction2 = "COPY#NERV6666";
        let invalid_instruction3 = "COPY #NERV";
        let invalid_instruction4 = "COPY";

        let expected1 = Ok(Instruction::Copy(
            Value::Number(-9999),
            Value::RegisterId("X".to_string()),
        ));
        let expected2 = Ok(Instruction::Copy(
            Value::RegisterId("T".to_string()),
            Value::RegisterId("X".to_string()),
        ));
        let expected3 = Ok(Instruction::Copy(
            Value::Number(666),
            Value::RegisterId("#NERV".to_string()),
        ));
        let expected4 = Ok(Instruction::Copy(
            Value::RegisterId("#NERV".to_string()),
            Value::RegisterId("X".to_string()),
        ));
        let expected_err1: Result<Instruction, ParseError> =
            Err(ParseError("COPY #NERV 6666".to_string()));
        let expected_err2: Result<Instruction, ParseError> =
            Err(ParseError("COPY#NERV6666".to_string()));
        let expected_err3: Result<Instruction, ParseError> =
            Err(ParseError("COPY #NERV".to_string()));
        let expected_err4: Result<Instruction, ParseError> = Err(ParseError("COPY".to_string()));

        let result1 = instruction1.parse();
        let result2 = instruction2.parse();
        let result3 = instruction3.parse();
        let result4 = instruction4.parse();
        let err1 = invalid_instruction1.parse();
        let err2 = invalid_instruction2.parse();
        let err3 = invalid_instruction3.parse();
        let err4 = invalid_instruction4.parse();

        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
        assert_eq!(result3, expected3);
        assert_eq!(result4, expected4);
        assert_eq!(err1, expected_err1);
        assert_eq!(err2, expected_err2);
        assert_eq!(err3, expected_err3);
        assert_eq!(err4, expected_err4);
    }

    #[test]
    fn test_parse_add() {
        unimplemented!()
    }

    #[test]
    fn test_parse_subtract() {
        unimplemented!()
    }

    #[test]
    fn test_parse_multiply() {
        unimplemented!()
    }

    #[test]
    fn test_parse_divide() {
        unimplemented!()
    }

    #[test]
    fn test_parse_modulo() {
        unimplemented!()
    }

    #[test]
    fn test_parse_swiz() {
        unimplemented!()
    }

    #[test]
    fn test_parse_mark() {
        unimplemented!()
    }

    #[test]
    fn test_parse_jump() {
        unimplemented!()
    }

    #[test]
    fn test_parse_jump_if_true() {
        unimplemented!()
    }

    #[test]
    fn test_parse_jump_if_false() {
        unimplemented!()
    }

    #[test]
    fn test_parse_test_equal() {
        unimplemented!()
    }

    #[test]
    fn test_parse_test_greater_than() {
        unimplemented!()
    }

    #[test]
    fn test_parse_test_less_than() {
        unimplemented!()
    }

    #[test]
    fn test_parse_replicate() {
        unimplemented!()
    }

    #[test]
    fn test_parse_halt() {
        unimplemented!()
    }

    #[test]
    fn test_parse_kill() {
        unimplemented!()
    }

    #[test]
    fn test_parse_link() {
        unimplemented!()
    }

    #[test]
    fn test_parse_host() {
        unimplemented!()
    }

    #[test]
    fn test_parse_mode() {
        unimplemented!()
    }

    #[test]
    fn test_parse_voidm() {
        unimplemented!()
    }

    #[test]
    fn test_parse_testmrd() {
        unimplemented!()
    }

    #[test]
    fn test_parse_make() {
        unimplemented!()
    }

    #[test]
    fn test_parse_grab() {
        unimplemented!()
    }

    #[test]
    fn test_parse_file() {
        unimplemented!()
    }

    #[test]
    fn test_parse_seek() {
        unimplemented!()
    }

    #[test]
    fn test_parse_voidf() {
        unimplemented!()
    }

    #[test]
    fn test_parse_drop() {
        unimplemented!()
    }

    #[test]
    fn test_parse_wipe() {
        unimplemented!()
    }

    #[test]
    fn test_parse_testeof() {
        unimplemented!()
    }

    #[test]
    fn test_parse_note() {
        unimplemented!()
    }

    #[test]
    fn test_parse_noop() {
        unimplemented!()
    }

    #[test]
    fn test_parse_random() {
        unimplemented!()
    }
}
