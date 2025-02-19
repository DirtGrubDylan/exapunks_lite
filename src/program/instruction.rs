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
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum ParseError {
    InvalidInstruction,
    InvalidLineLength,
    InvalidValues,
    InvalidTestOperation,
    MissingTestOperation,
}

impl Instruction {
    /// Parses a given line to a `RegisterId`/`Number` and applies the constructor.
    ///
    /// A valid line is "[instruction] [first value]".
    ///
    /// * The instruction has to be 4 character, but is ignored in this method.
    /// * The first value has to be a valid [`Value::RegisterId`] or [`Value::Number`].
    ///
    /// # Errors
    ///
    /// Returns an error if the line:
    ///
    /// * Is not 2 distinct words seperated by a space.
    /// * Doesn't have a valid register id and/or number as the first value.
    fn parse_rn<C>(line: &str, constructor: C) -> Result<Self, ParseError>
    where
        C: Fn(Value) -> Self,
    {
        let split_line: Vec<&str> = line.split(' ').collect();

        if split_line.len() != 2 {
            return Err(ParseError::InvalidLineLength);
        }

        Value::new_number_or_register_id(split_line[1])
            .map(constructor)
            .map_err(|_| ParseError::InvalidValues)
    }

    /// Parses a given line to a (`RegisterId`/`Number`, `RegisterId`) and applies the constructor.
    ///
    /// A valid line is "[instruction] [first value] [second value]".
    ///
    /// * The instruction has to be 4 character, but is ignored in this method.
    /// * The first value has to be a valid [`Value::RegisterId`] or [`Value::Number`].
    /// * The second value has to be a valid [`Value::RegisterId`].
    ///
    /// # Errors
    ///
    /// Returns an error if the line:
    ///
    /// * Is not 3 distinct words seperated by a space.
    /// * Doesn't have a valid register id and/or number as the first value.
    /// * Doesn't have a valid register id as the second value.
    fn parse_rn_r<C>(line: &str, constructor: C) -> Result<Self, ParseError>
    where
        C: Fn(Value, Value) -> Self,
    {
        let split_line: Vec<&str> = line.split(' ').collect();

        if split_line.len() != 3 {
            return Err(ParseError::InvalidLineLength);
        }

        let source_result = Value::new_number_or_register_id(split_line[1]);
        let destination_result = Value::new_register_id(split_line[2]);

        match (source_result, destination_result) {
            (Ok(source), Ok(destination)) => Ok(constructor(source, destination)),
            _ => Err(ParseError::InvalidValues),
        }
    }

    /// Parses a given line to a (`RegisterId`/`Number`, `RegisterId`/`Number`, `RegisterId`) and applies the constructor.
    ///
    /// A valid line is "[instruction] [first value] [second value] [third value]".
    ///
    /// * The instruction has to be 4 character, but is ignored in this method.
    /// * The first value has to be a valid [`Value::RegisterId`] or [`Value::Number`].
    /// * The second value has to be a valid [`Value::RegisterId`] or [`Value::Number`].
    /// * The third value has to be a valid [`Value::RegisterId`].
    ///
    /// # Errors
    ///
    /// Returns an error if the line:
    ///
    /// * Is not 4 distinct words seperated by a space.
    /// * Doesn't have a valid register id and/or number as the first value.
    /// * Doesn't have a valid register id and/or number as the second value.
    /// * Doesn't have a valid register id as the third value.
    fn parse_rn_rn_r<C>(line: &str, constructor: C) -> Result<Self, ParseError>
    where
        C: Fn(Value, Value, Value) -> Self,
    {
        let split_line: Vec<&str> = line.split(' ').collect();

        if split_line.len() != 4 {
            return Err(ParseError::InvalidLineLength);
        }

        let first_result = Value::new_number_or_register_id(split_line[1]);
        let second_result = Value::new_number_or_register_id(split_line[2]);
        let destination_result = Value::new_register_id(split_line[3]);

        match (first_result, second_result, destination_result) {
            (Ok(first_source), Ok(second_source), Ok(destination)) => {
                Ok(constructor(first_source, second_source, destination))
            }
            _ => Err(ParseError::InvalidValues),
        }
    }

    /// Parses a given line to a `RegisterId` and applies the constructor.
    ///
    /// A valid line is "[instruction] [first value]".
    ///
    /// * The instruction has to be 4 character, but is ignored in this method.
    /// * The first value has to be a valid [`Value::RegisterId`].
    ///
    /// # Errors
    ///
    /// Returns an error if the line:
    ///
    /// * Is not 2 distinct words seperated by a space.
    /// * Doesn't have a valid register id as the first value.
    fn parse_r<C>(line: &str, constructor: C) -> Result<Self, ParseError>
    where
        C: Fn(Value) -> Self,
    {
        let split_line: Vec<&str> = line.split(' ').collect();

        if split_line.len() != 2 {
            return Err(ParseError::InvalidLineLength);
        }

        Value::new_register_id(split_line[1])
            .map(constructor)
            .map_err(|_| ParseError::InvalidValues)
    }

    /// Parses a given line to a `LabelId` and applies the constructor.
    ///
    /// A valid line is "[instruction] [first value]".
    ///
    /// * The instruction has to be 4 character, but is ignored in this method.
    /// * The first value has to be a valid [`Value::LabelId`].
    ///
    /// # Errors
    ///
    /// Returns an error if the line:
    ///
    /// * Is not 2 distinct words seperated by a space.
    /// * Doesn't have a valid label id as the first value.
    fn parse_l<C>(line: &str, constructor: C) -> Result<Self, ParseError>
    where
        C: Fn(Value) -> Self,
    {
        let split_line: Vec<&str> = line.split(' ').collect();

        if split_line.len() != 2 {
            return Err(ParseError::InvalidLineLength);
        }

        Value::new_label_id(split_line[1])
            .map(constructor)
            .map_err(|_| ParseError::InvalidValues)
    }

    /// Parses a given test line to an instruction.
    ///
    /// A valid line is "[instruction] [first value] [=><] [second value]".
    ///
    /// * The instruction has to be 4 character, but is ignored in this method.
    /// * The first value has to be a valid [`Value::RegisterId`] or [`Value::Number`].
    /// * The second value has to be a valid [`Value::RegisterId`] or [`Value::Number`].
    ///
    /// # Errors
    ///
    /// Returns an error if the line:
    ///
    /// * Is not 4 distinct words seperated by a space.
    /// * Doesn't have a valid register id and/or number as the first value.
    /// * Doesn't have a valid register id and/or number as the second value.
    /// * Doesn't have a valid operation (i.e. '=', '>', or '<').
    fn parse_test(line: &str) -> Result<Self, ParseError> {
        let split_line: Vec<&str> = line.split(' ').collect();

        if split_line.len() != 4 {
            return Err(ParseError::InvalidLineLength);
        } else if !matches!(split_line[2], "=" | ">" | "<") {
            return Err(ParseError::InvalidTestOperation);
        }

        let first_source_result = Value::new_number_or_register_id(split_line[1]);
        let second_source_result = Value::new_number_or_register_id(split_line[3]);

        match (first_source_result, second_source_result) {
            (Ok(first_source), Ok(second_source)) if (split_line[2] == "=") => {
                Ok(Self::TestEqual(first_source, second_source))
            }
            (Ok(first_source), Ok(second_source)) if (split_line[2] == ">") => {
                Ok(Self::TestGreaterThan(first_source, second_source))
            }
            (Ok(first_source), Ok(second_source)) if (split_line[2] == "<") => {
                Ok(Self::TestLessThan(first_source, second_source))
            }
            _ => Err(ParseError::InvalidValues),
        }
    }

    /// Parses to a single given instruction.
    ///
    /// A valid single instruction is "[instruction]".
    ///
    /// * The instruction has to be 4 characters.
    ///
    /// # Errors
    ///
    /// Returns an error if the line:
    ///
    /// * Is not a single word.
    /// * Is empty.
    fn parse_single_instruction(line: &str, instruction: Self) -> Result<Self, ParseError> {
        if line.len() == 4 {
            Ok(instruction)
        } else {
            Err(ParseError::InvalidLineLength)
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let error = Err(ParseError::InvalidInstruction);
        let instruction: &str = line.split(' ').next().unwrap_or("");

        match instruction {
            "COPY" => Self::parse_rn_r(line, Self::Copy),
            "ADDI" => Self::parse_rn_rn_r(line, Self::Add),
            "SUBI" => Self::parse_rn_rn_r(line, Self::Subtract),
            "MULI" => Self::parse_rn_rn_r(line, Self::Multiply),
            "DIVI" => Self::parse_rn_rn_r(line, Self::Divide),
            "MODI" => Self::parse_rn_rn_r(line, Self::Modulo),
            "SWIZ" => Self::parse_rn_rn_r(line, Self::Swiz),
            "MARK" => Self::parse_l(line, Self::Mark),
            "JUMP" => Self::parse_l(line, Self::Jump),
            "TJMP" => Self::parse_l(line, Self::JumpIfTrue),
            "FJMP" => Self::parse_l(line, Self::JumpIfFalse),
            "TEST" if (line == "TEST MRD") => Ok(Self::TestMRD),
            "TEST" if (line == "TEST EOF") => Ok(Self::TestEndOfFile),
            "TEST" => Self::parse_test(line),
            "REPL" => Self::parse_l(line, Self::Replicate),
            "HALT" => Self::parse_single_instruction(line, Self::Halt),
            "KILL" => Self::parse_single_instruction(line, Self::Kill),
            "LINK" => Self::parse_rn(line, Self::Link),
            "HOST" => Self::parse_r(line, Self::Host),
            "MODE" => Self::parse_single_instruction(line, Self::Mode),
            "VOID" if (line == "VOID M") => Ok(Self::VoidM),
            "MAKE" => Self::parse_single_instruction(line, Self::Make),
            "GRAB" => Self::parse_rn(line, Self::Grab),
            "FILE" => Self::parse_r(line, Self::File),
            "SEEK" => Self::parse_rn(line, Self::Seek),
            "VOID" if (line == "VOID F") => Ok(Self::VoidF),
            "DROP" => Self::parse_single_instruction(line, Self::Drop),
            "WIPE" => Self::parse_single_instruction(line, Self::Wipe),
            "NOTE" => Ok(Self::Note),
            "NOOP" => Self::parse_single_instruction(line, Self::NoOp),
            "RAND" => Self::parse_rn_rn_r(line, Self::Random),
            _ => error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Instruction, ParseError, Value};

    #[test]
    fn test_parse_empty() {
        let empty_instruction = "";

        let expected_err: Result<Instruction, ParseError> = Err(ParseError::InvalidInstruction);

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
        let invalid_instruction5 = "COPY 6666 #NERVX";

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
        let expected_err1: Result<Instruction, ParseError> = Err(ParseError::InvalidValues);
        let expected_err2: Result<Instruction, ParseError> = Err(ParseError::InvalidInstruction);
        let expected_err3: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err4: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err5: Result<Instruction, ParseError> = Err(ParseError::InvalidValues);

        let result1 = instruction1.parse();
        let result2 = instruction2.parse();
        let result3 = instruction3.parse();
        let result4 = instruction4.parse();
        let err1 = invalid_instruction1.parse();
        let err2 = invalid_instruction2.parse();
        let err3 = invalid_instruction3.parse();
        let err4 = invalid_instruction4.parse();
        let err5 = invalid_instruction5.parse();

        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
        assert_eq!(result3, expected3);
        assert_eq!(result4, expected4);
        assert_eq!(err1, expected_err1);
        assert_eq!(err2, expected_err2);
        assert_eq!(err3, expected_err3);
        assert_eq!(err4, expected_err4);
        assert_eq!(err5, expected_err5);
    }

    #[test]
    fn test_parse_add() {
        let instruction1 = "ADDI -9999 X X";
        let instruction2 = "ADDI T X #NERV";
        let instruction3 = "ADDI 666 1 #NERV";
        let instruction4 = "ADDI #NERV -666 X";
        let invalid_instruction1 = "ADDI -9999 X 6666";
        let invalid_instruction2 = "ADDIXT#NERV";
        let invalid_instruction3 = "ADDI X #NERV";
        let invalid_instruction4 = "ADDI";
        let invalid_instruction5 = "ADDI 6666 1 #NERVX";

        let expected1 = Ok(Instruction::Add(
            Value::Number(-9999),
            Value::RegisterId("X".to_string()),
            Value::RegisterId("X".to_string()),
        ));
        let expected2 = Ok(Instruction::Add(
            Value::RegisterId("T".to_string()),
            Value::RegisterId("X".to_string()),
            Value::RegisterId("#NERV".to_string()),
        ));
        let expected3 = Ok(Instruction::Add(
            Value::Number(666),
            Value::Number(1),
            Value::RegisterId("#NERV".to_string()),
        ));
        let expected4 = Ok(Instruction::Add(
            Value::RegisterId("#NERV".to_string()),
            Value::Number(-666),
            Value::RegisterId("X".to_string()),
        ));
        let expected_err1: Result<Instruction, ParseError> = Err(ParseError::InvalidValues);
        let expected_err2: Result<Instruction, ParseError> = Err(ParseError::InvalidInstruction);
        let expected_err3: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err4: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err5: Result<Instruction, ParseError> = Err(ParseError::InvalidValues);

        let result1 = instruction1.parse();
        let result2 = instruction2.parse();
        let result3 = instruction3.parse();
        let result4 = instruction4.parse();
        let err1 = invalid_instruction1.parse();
        let err2 = invalid_instruction2.parse();
        let err3 = invalid_instruction3.parse();
        let err4 = invalid_instruction4.parse();
        let err5 = invalid_instruction5.parse();

        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
        assert_eq!(result3, expected3);
        assert_eq!(result4, expected4);
        assert_eq!(err1, expected_err1);
        assert_eq!(err2, expected_err2);
        assert_eq!(err3, expected_err3);
        assert_eq!(err4, expected_err4);
        assert_eq!(err5, expected_err5);
    }

    #[test]
    fn test_parse_mark() {
        let instruction1 = "MARK LABEL";
        let instruction2 = "MARK -666";
        let instruction3 = "MARK #NERV";
        let instruction4 = "MARK 666";
        let invalid_instruction1 = "MARK -9999 LABEL";
        let invalid_instruction2 = "MARKLABEL";
        let invalid_instruction3 = "MARK";
        let invalid_instruction4 = "MARK ";

        let expected1 = Ok(Instruction::Mark(Value::LabelId("LABEL".to_string())));
        let expected2 = Ok(Instruction::Mark(Value::LabelId("-666".to_string())));
        let expected3 = Ok(Instruction::Mark(Value::LabelId("#NERV".to_string())));
        let expected4 = Ok(Instruction::Mark(Value::LabelId("666".to_string())));
        let expected_err1: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err2: Result<Instruction, ParseError> = Err(ParseError::InvalidInstruction);
        let expected_err3: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err4: Result<Instruction, ParseError> = Err(ParseError::InvalidValues);

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
    fn test_parse_test() {
        let instruction1 = "TEST -9999 = X";
        let instruction2 = "TEST #NERV = X";
        let instruction3 = "TEST #NERV = 6666";
        let instruction4 = "TEST -666 = 6666";
        let instruction5 = "TEST -9999 > X";
        let instruction6 = "TEST #NERV < X";
        let invalid_instruction1 = "TEST -9999 = Y";
        let invalid_instruction2 = "TEST-9999=X";
        let invalid_instruction3 = "TEST -9999 =";
        let invalid_instruction4 = "TEST";
        let invalid_instruction5 = "TEST 6666 = #NERVX";
        let invalid_instruction6 = "TEST -9999 X";
        let invalid_instruction7 = "TEST -9999 >= X";
        let invalid_instruction8 = "TEST -9999 X X";

        let expected1 = Ok(Instruction::TestEqual(
            Value::Number(-9999),
            Value::RegisterId("X".to_string()),
        ));
        let expected2 = Ok(Instruction::TestEqual(
            Value::RegisterId("#NERV".to_string()),
            Value::RegisterId("X".to_string()),
        ));
        let expected3 = Ok(Instruction::TestEqual(
            Value::RegisterId("#NERV".to_string()),
            Value::Number(6666),
        ));
        let expected4 = Ok(Instruction::TestEqual(
            Value::Number(-666),
            Value::Number(6666),
        ));
        let expected5 = Ok(Instruction::TestGreaterThan(
            Value::Number(-9999),
            Value::RegisterId("X".to_string()),
        ));
        let expected6 = Ok(Instruction::TestLessThan(
            Value::RegisterId("#NERV".to_string()),
            Value::RegisterId("X".to_string()),
        ));
        let expected_err1: Result<Instruction, ParseError> = Err(ParseError::InvalidValues);
        let expected_err2: Result<Instruction, ParseError> = Err(ParseError::InvalidInstruction);
        let expected_err3: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err4: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err5: Result<Instruction, ParseError> = Err(ParseError::InvalidValues);
        let expected_err6: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err7: Result<Instruction, ParseError> = Err(ParseError::InvalidTestOperation);
        let expected_err8: Result<Instruction, ParseError> = Err(ParseError::InvalidTestOperation);

        let result1 = instruction1.parse();
        let result2 = instruction2.parse();
        let result3 = instruction3.parse();
        let result4 = instruction4.parse();
        let result5 = instruction5.parse();
        let result6 = instruction6.parse();
        let err1 = invalid_instruction1.parse();
        let err2 = invalid_instruction2.parse();
        let err3 = invalid_instruction3.parse();
        let err4 = invalid_instruction4.parse();
        let err5 = invalid_instruction5.parse();
        let err6 = invalid_instruction6.parse();
        let err7 = invalid_instruction7.parse();
        let err8 = invalid_instruction8.parse();

        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
        assert_eq!(result3, expected3);
        assert_eq!(result4, expected4);
        assert_eq!(result5, expected5);
        assert_eq!(result6, expected6);
        assert_eq!(err1, expected_err1);
        assert_eq!(err2, expected_err2);
        assert_eq!(err3, expected_err3);
        assert_eq!(err4, expected_err4);
        assert_eq!(err5, expected_err5);
        assert_eq!(err6, expected_err6);
        assert_eq!(err7, expected_err7);
        assert_eq!(err8, expected_err8);
    }

    #[test]
    fn test_parse_halt() {
        let instruction = "HALT";
        let invalid_instruction1 = "HALT 666";
        let invalid_instruction2 = "HALTT";
        let invalid_instruction3 = "HALT ";

        let expected = Ok(Instruction::Halt);
        let expected_err1: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err2: Result<Instruction, ParseError> = Err(ParseError::InvalidInstruction);
        let expected_err3: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);

        let result = instruction.parse();
        let err1 = invalid_instruction1.parse();
        let err2 = invalid_instruction2.parse();
        let err3 = invalid_instruction3.parse();

        assert_eq!(result, expected);
        assert_eq!(err1, expected_err1);
        assert_eq!(err2, expected_err2);
        assert_eq!(err3, expected_err3);
    }

    #[test]
    fn test_parse_link() {
        let instruction1 = "LINK 666";
        let instruction2 = "LINK X";
        let instruction3 = "LINK #NERV";
        let invalid_instruction1 = "LINK -9999 X";
        let invalid_instruction2 = "LINK #NERVX";
        let invalid_instruction3 = "LINK";
        let invalid_instruction4 = "LINK Y";

        let expected1 = Ok(Instruction::Link(Value::Number(666)));
        let expected2 = Ok(Instruction::Link(Value::RegisterId("X".to_string())));
        let expected3 = Ok(Instruction::Link(Value::RegisterId("#NERV".to_string())));
        let expected_err1: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err2: Result<Instruction, ParseError> = Err(ParseError::InvalidValues);
        let expected_err3: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err4: Result<Instruction, ParseError> = Err(ParseError::InvalidValues);

        let result1 = instruction1.parse();
        let result2 = instruction2.parse();
        let result3 = instruction3.parse();
        let err1 = invalid_instruction1.parse();
        let err2 = invalid_instruction2.parse();
        let err3 = invalid_instruction3.parse();
        let err4 = invalid_instruction4.parse();

        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
        assert_eq!(result3, expected3);
        assert_eq!(err1, expected_err1);
        assert_eq!(err2, expected_err2);
        assert_eq!(err3, expected_err3);
        assert_eq!(err4, expected_err4);
    }

    #[test]
    fn test_parse_host() {
        let instruction1 = "HOST X";
        let instruction2 = "HOST #NERV";
        let invalid_instruction1 = "HOST -9999";
        let invalid_instruction2 = "HOST #NERVX";
        let invalid_instruction3 = "HOST X #NERV";
        let invalid_instruction4 = "HOST Y";

        let expected1 = Ok(Instruction::Host(Value::RegisterId("X".to_string())));
        let expected2 = Ok(Instruction::Host(Value::RegisterId("#NERV".to_string())));
        let expected_err1: Result<Instruction, ParseError> = Err(ParseError::InvalidValues);
        let expected_err2: Result<Instruction, ParseError> = Err(ParseError::InvalidValues);
        let expected_err3: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err4: Result<Instruction, ParseError> = Err(ParseError::InvalidValues);

        let result1 = instruction1.parse();
        let result2 = instruction2.parse();
        let err1 = invalid_instruction1.parse();
        let err2 = invalid_instruction2.parse();
        let err3 = invalid_instruction3.parse();
        let err4 = invalid_instruction4.parse();

        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
        assert_eq!(err1, expected_err1);
        assert_eq!(err2, expected_err2);
        assert_eq!(err3, expected_err3);
        assert_eq!(err4, expected_err4);
    }

    #[test]
    fn test_parse_void() {
        let instruction1 = "VOID M";
        let instruction2 = "VOID F";
        let invalid_instruction1 = "VOID X";
        let invalid_instruction2 = "VOID #NERV";
        let invalid_instruction3 = "VOID M #NERV";
        let invalid_instruction4 = "VOID 666";

        let expected1 = Ok(Instruction::VoidM);
        let expected2 = Ok(Instruction::VoidF);
        let expected_err1: Result<Instruction, ParseError> = Err(ParseError::InvalidInstruction);
        let expected_err2: Result<Instruction, ParseError> = Err(ParseError::InvalidInstruction);
        let expected_err3: Result<Instruction, ParseError> = Err(ParseError::InvalidInstruction);
        let expected_err4: Result<Instruction, ParseError> = Err(ParseError::InvalidInstruction);

        let result1 = instruction1.parse();
        let result2 = instruction2.parse();
        let err1 = invalid_instruction1.parse();
        let err2 = invalid_instruction2.parse();
        let err3 = invalid_instruction3.parse();
        let err4 = invalid_instruction4.parse();

        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
        assert_eq!(err1, expected_err1);
        assert_eq!(err2, expected_err2);
        assert_eq!(err3, expected_err3);
        assert_eq!(err4, expected_err4);
    }

    #[test]
    fn test_parse_testmrd_testeof() {
        let instruction1 = "TEST MRD";
        let instruction2 = "TEST EOF";
        let invalid_instruction1 = "TEST MRDD";
        let invalid_instruction2 = "TEST EOFF";
        let invalid_instruction3 = "TEST MR";
        let invalid_instruction4 = "TEST EO";

        let expected1 = Ok(Instruction::TestMRD);
        let expected2 = Ok(Instruction::TestEndOfFile);
        let expected_err1: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err2: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err3: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);
        let expected_err4: Result<Instruction, ParseError> = Err(ParseError::InvalidLineLength);

        let result1 = instruction1.parse();
        let result2 = instruction2.parse();
        let err1 = invalid_instruction1.parse();
        let err2 = invalid_instruction2.parse();
        let err3 = invalid_instruction3.parse();
        let err4 = invalid_instruction4.parse();

        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
        assert_eq!(err1, expected_err1);
        assert_eq!(err2, expected_err2);
        assert_eq!(err3, expected_err3);
        assert_eq!(err4, expected_err4);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::similar_names)]
    fn test_parse_all() {
        let copy_string = "COPY 1 X";
        let add_string = "ADDI 1 X X";
        let subtract_string = "SUBI X F X";
        let multiply_string = "MULI #NERV 2 F";
        let divide_string = "DIVI -4444 4 X";
        let modulo_string = "MODI T X T";
        let swiz_string = "SWIZ 6789 4321 X";
        let mark_string = "MARK THIS_LABEL";
        let jump_string = "JUMP THIS_LABEL";
        let jump_if_true_string = "TJMP THIS_LABEL";
        let jump_if_false_string = "FJMP THIS_LABEL";
        let test_equal_string = "TEST X = 4";
        let test_greater_than_string = "TEST 4 > #NERV";
        let test_less_than_string = "TEST #NERV < X";
        let replicate_string = "REPL THIS_LABEL";
        let halt_string = "HALT";
        let kill_string = "KILL";
        let link_string = "LINK 800";
        let host_string = "HOST F";
        let mode_string = "MODE";
        let void_m_string = "VOID M";
        let test_mrd_string = "TEST MRD";
        let make_string = "MAKE";
        let grab_string = "GRAB 200";
        let file_string = "FILE X";
        let seek_string = "SEEK #NERV";
        let void_f_string = "VOID F";
        let drop_string = "DROP";
        let wipe_string = "WIPE";
        let test_eof_string = "TEST EOF";
        let note_string = "NOTE here is a dumb note";
        let noop_string = "NOOP";
        let rand_string = "RAND 2 F #RAND";

        let copy = Ok(Instruction::Copy(
            Value::Number(1),
            Value::RegisterId("X".to_string()),
        ));
        let add = Ok(Instruction::Add(
            Value::Number(1),
            Value::RegisterId("X".to_string()),
            Value::RegisterId("X".to_string()),
        ));
        let subtract = Ok(Instruction::Subtract(
            Value::RegisterId("X".to_string()),
            Value::RegisterId("F".to_string()),
            Value::RegisterId("X".to_string()),
        ));
        let multiply = Ok(Instruction::Multiply(
            Value::RegisterId("#NERV".to_string()),
            Value::Number(2),
            Value::RegisterId("F".to_string()),
        ));
        let divide = Ok(Instruction::Divide(
            Value::Number(-4444),
            Value::Number(4),
            Value::RegisterId("X".to_string()),
        ));
        let modulo = Ok(Instruction::Modulo(
            Value::RegisterId("T".to_string()),
            Value::RegisterId("X".to_string()),
            Value::RegisterId("T".to_string()),
        ));
        let swiz = Ok(Instruction::Swiz(
            Value::Number(6789),
            Value::Number(4321),
            Value::RegisterId("X".to_string()),
        ));
        let mark = Ok(Instruction::Mark(Value::LabelId("THIS_LABEL".to_string())));
        let jump = Ok(Instruction::Jump(Value::LabelId("THIS_LABEL".to_string())));
        let jump_if_true = Ok(Instruction::JumpIfTrue(Value::LabelId(
            "THIS_LABEL".to_string(),
        )));
        let jump_if_false = Ok(Instruction::JumpIfFalse(Value::LabelId(
            "THIS_LABEL".to_string(),
        )));
        let test_equal = Ok(Instruction::TestEqual(
            Value::RegisterId("X".to_string()),
            Value::Number(4),
        ));
        let test_greater_than = Ok(Instruction::TestGreaterThan(
            Value::Number(4),
            Value::RegisterId("#NERV".to_string()),
        ));
        let test_less_than = Ok(Instruction::TestLessThan(
            Value::RegisterId("#NERV".to_string()),
            Value::RegisterId("X".to_string()),
        ));
        let replicate = Ok(Instruction::Replicate(Value::LabelId(
            "THIS_LABEL".to_string(),
        )));
        let halt = Ok(Instruction::Halt);
        let kill = Ok(Instruction::Kill);
        let link = Ok(Instruction::Link(Value::Number(800)));
        let host = Ok(Instruction::Host(Value::RegisterId("F".to_string())));
        let mode = Ok(Instruction::Mode);
        let void_m = Ok(Instruction::VoidM);
        let test_mrd = Ok(Instruction::TestMRD);
        let make = Ok(Instruction::Make);
        let grab = Ok(Instruction::Grab(Value::Number(200)));
        let file = Ok(Instruction::File(Value::RegisterId("X".to_string())));
        let seek = Ok(Instruction::Seek(Value::RegisterId("#NERV".to_string())));
        let void_f = Ok(Instruction::VoidF);
        let drop = Ok(Instruction::Drop);
        let wipe = Ok(Instruction::Wipe);
        let test_eof = Ok(Instruction::TestEndOfFile);
        let note = Ok(Instruction::Note);
        let noop = Ok(Instruction::NoOp);
        let rand = Ok(Instruction::Random(
            Value::Number(2),
            Value::RegisterId("F".to_string()),
            Value::RegisterId("#RAND".to_string()),
        ));

        let copy_result = copy_string.parse();
        let add_result = add_string.parse();
        let subtract_result = subtract_string.parse();
        let multiply_result = multiply_string.parse();
        let divide_result = divide_string.parse();
        let modulo_result = modulo_string.parse();
        let swiz_result = swiz_string.parse();
        let mark_result = mark_string.parse();
        let jump_result = jump_string.parse();
        let jump_if_true_result = jump_if_true_string.parse();
        let jump_if_false_result = jump_if_false_string.parse();
        let test_equal_result = test_equal_string.parse();
        let test_greater_than_result = test_greater_than_string.parse();
        let test_less_than_result = test_less_than_string.parse();
        let replicate_result = replicate_string.parse();
        let halt_result = halt_string.parse();
        let kill_result = kill_string.parse();
        let link_result = link_string.parse();
        let host_result = host_string.parse();
        let mode_result = mode_string.parse();
        let void_m_result = void_m_string.parse();
        let test_mrd_result = test_mrd_string.parse();
        let make_result = make_string.parse();
        let grab_result = grab_string.parse();
        let file_result = file_string.parse();
        let seek_result = seek_string.parse();
        let void_f_result = void_f_string.parse();
        let drop_result = drop_string.parse();
        let wipe_result = wipe_string.parse();
        let test_eof_result = test_eof_string.parse();
        let note_result = note_string.parse();
        let noop_result = noop_string.parse();
        let rand_result = rand_string.parse();

        assert_eq!(copy_result, copy);
        assert_eq!(add_result, add);
        assert_eq!(subtract_result, subtract);
        assert_eq!(multiply_result, multiply);
        assert_eq!(divide_result, divide);
        assert_eq!(modulo_result, modulo);
        assert_eq!(swiz_result, swiz);
        assert_eq!(mark_result, mark);
        assert_eq!(jump_result, jump);
        assert_eq!(jump_if_true_result, jump_if_true);
        assert_eq!(jump_if_false_result, jump_if_false);
        assert_eq!(test_equal_result, test_equal);
        assert_eq!(test_greater_than_result, test_greater_than);
        assert_eq!(test_less_than_result, test_less_than);
        assert_eq!(replicate_result, replicate);
        assert_eq!(halt_result, halt);
        assert_eq!(kill_result, kill);
        assert_eq!(link_result, link);
        assert_eq!(host_result, host);
        assert_eq!(mode_result, mode);
        assert_eq!(void_m_result, void_m);
        assert_eq!(test_mrd_result, test_mrd);
        assert_eq!(make_result, make);
        assert_eq!(grab_result, grab);
        assert_eq!(file_result, file);
        assert_eq!(seek_result, seek);
        assert_eq!(void_f_result, void_f);
        assert_eq!(drop_result, drop);
        assert_eq!(wipe_result, wipe);
        assert_eq!(test_eof_result, test_eof);
        assert_eq!(note_result, note);
        assert_eq!(noop_result, noop);
        assert_eq!(rand_result, rand);
    }
}
