use std::collections::HashMap;

use crate::instruction;
use crate::instruction::Instruction;
use crate::util::file_reader::to_string_vector;
use crate::value::Value;

/// A Program is just a stack of [`Instruction`]s.
///
/// It can jump to certain instructions, it can return the next instruction in the stack, and keeps
/// track of instruction index.
///
/// These can be created manually or via a customer *.exa file type.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Program {
    file_path: String,
    instructions: HashMap<usize, Instruction>,
    marks: HashMap<String, usize>,
    stack_index: usize,
}

/// A dummy struct to indicate which line number had errors.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum LineParseError {
    InvalidInstruction(usize, instruction::ParseError),
    MissingMarkLabel(usize, String),
}

impl LineParseError {
    /// Fetches the line number.
    fn line_number(&self) -> usize {
        match self {
            Self::InvalidInstruction(line_number, _) => *line_number,
            Self::MissingMarkLabel(line_number, _) => *line_number,
        }
    }
}

/// A dummy struct to indicate which line number had errors.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct ParseError(Vec<LineParseError>);

impl Program {
    /// Instantiate a Program from a given list of [`String`]s.
    ///
    /// The each item in the list will be converted to an [`Instruction`].
    ///
    /// # Errors
    ///
    /// Errors will be returned as a list of line numbers and the specific errors.
    pub fn new(instruction_strings: &[String]) -> Result<Self, ParseError> {
        let mut marks = HashMap::new();
        let mut instructions = HashMap::new();
        let mut instruction_parse_results = HashMap::new();

        for (line_number, line) in instruction_strings.iter().enumerate() {
            // Lines starting with '#' is a comment.
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            let parse_result: Result<Instruction, instruction::ParseError> = line.parse();

            instruction_parse_results.insert(line_number, parse_result.clone());

            match parse_result {
                Ok(Instruction::Mark(Value::LabelId(label))) => {
                    marks.insert(label.clone(), line_number);
                }
                Ok(instruction) => {
                    instructions.insert(line_number, instruction.clone());
                }
                _ => {}
            }
        }

        let possible_parse_error = Self::parse_error(&instruction_parse_results, &marks);

        let program = Program {
            file_path: String::new(),
            instructions,
            marks,
            stack_index: 0,
        };

        match possible_parse_error {
            Some(error) => Err(error),
            None => Ok(program),
        }
    }

    /// Instantiate a Program from a given *.exa file name.
    ///
    /// This will read the file and try to parse to a Program.
    ///
    /// # Errors
    ///
    /// Errors will be returned as a list of line numbers and the specific errors.
    pub fn new_from_file(file_name: &str) -> Result<Self, ParseError> {
        if !file_name.ends_with(".exa") {
            panic!("File {file_name} is invalid, and must end with '.exa'");
        }

        Program::new(&to_string_vector(file_name).unwrap()).map(|mut program| {
            program.file_path = file_name.to_string();

            program
        })
    }

    /// Creates a possible [`ParseError`] for the given list of [`Instruction`]s and seen `MARK`
    /// labels.
    fn parse_error(
        parse_results: &HashMap<usize, Result<Instruction, instruction::ParseError>>,
        marks: &HashMap<String, usize>,
    ) -> Option<ParseError> {
        let mut errors = Vec::new();

        for (line_number, result) in parse_results {
            match result {
                Ok(
                    Instruction::Jump(Value::LabelId(label))
                    | Instruction::JumpIfTrue(Value::LabelId(label))
                    | Instruction::JumpIfFalse(Value::LabelId(label))
                    | Instruction::Replicate(Value::LabelId(label)),
                ) if !marks.contains_key(label) => {
                    errors.push(LineParseError::MissingMarkLabel(
                        *line_number,
                        label.clone(),
                    ));
                }
                Err(error) => {
                    errors.push(LineParseError::InvalidInstruction(*line_number, *error));
                }
                _ => {}
            }
        }

        errors.sort_by_key(|error| error.line_number());

        if errors.is_empty() {
            None
        } else {
            Some(ParseError(errors))
        }
    }
}

impl<const N: usize> TryFrom<[&str; N]> for Program {
    type Error = ParseError;

    fn try_from(input: [&str; N]) -> Result<Self, Self::Error> {
        let strings: Vec<String> = input.iter().map(ToString::to_string).collect();

        Self::try_from(strings.as_slice())
    }
}

impl TryFrom<&[String]> for Program {
    type Error = ParseError;

    fn try_from(input: &[String]) -> Result<Self, Self::Error> {
        Self::new(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_str_array_ok() {
        let instructions = vec![
            String::from("LINK 800"),
            String::new(),
            String::from("MARK THIS_LABEL"),
            String::from("# Grabbing the 200 file"),
            String::from("GRAB 200"),
            String::from("COPY F X"),
            String::from("HALT"),
        ];

        let expected_instructions = vec![
            String::from("LINK 800"),
            String::new(),
            String::new(),
            String::new(),
            String::from("GRAB 200"),
            String::from("COPY F X"),
            String::from("HALT"),
        ];

        let expected_program = Program {
            file_path: String::new(),
            instructions: expected_instructions
                .iter()
                .enumerate()
                .filter(|(_, s)| !s.is_empty())
                .filter_map(|(i, s)| s.parse().map(|instruction| (i, instruction)).ok())
                .collect(),
            marks: HashMap::from([(String::from("THIS_LABEL"), 2)]),
            stack_index: 0,
        };

        let program = Program::try_from(instructions.as_slice());

        assert_eq!(program, Ok(expected_program));
    }

    #[test]
    fn test_try_from_str_array_err() {
        let instructions = vec![
            String::from("LINK 800 LINK 800"),
            String::new(),
            String::from("GRAB 200"),
            String::from("COPY F 200"),
            String::new(),
            String::new(),
            String::from("JUMP THIS_LABEL"),
        ];

        let expected_error = ParseError(vec![
            LineParseError::InvalidInstruction(0, instruction::ParseError::InvalidLineLength),
            LineParseError::InvalidInstruction(3, instruction::ParseError::InvalidValues),
            LineParseError::MissingMarkLabel(6, String::from("THIS_LABEL")),
        ]);

        let program = Program::try_from(instructions.as_slice());

        assert_eq!(program, Err(expected_error));
    }

    #[test]
    fn test_new_from_file() {
        let expected_instructions = HashMap::from([
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
        ]);

        let expected_program = Program {
            file_path: String::from("test_files/simple_program.exa"),
            instructions: expected_instructions,
            marks: HashMap::from([(String::from("THIS_LABEL"), 5)]),
            stack_index: 0,
        };

        let program = Program::new_from_file("test_files/simple_program.exa");

        assert_eq!(program, Ok(expected_program));
    }
}
