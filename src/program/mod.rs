pub mod instruction;

use std::collections::HashMap;
use std::fmt;
use std::path::Path;

use crate::util::file_reader::to_string_vector;
use crate::value::Value;

use instruction::Instruction;

/// A Program is just a stack of [`Instruction`]s.
///
/// It can jump to certain instructions, it can return the next instruction in the stack, and keeps
/// track of instruction index.
///
/// These can be created manually or via a customer *.exa file type.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct Program {
    file_path: String,
    raw_lines: Vec<String>,
    instructions: Vec<(usize, Instruction)>,
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
            Self::InvalidInstruction(line_number, _) | Self::MissingMarkLabel(line_number, _) => {
                *line_number
            }
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
        let mut instructions = Vec::new();
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
                    // Instead of inserting the line number, it's more efficient to store the
                    // subsequent instruction index... which is the current length of the
                    // instructions list.
                    marks.insert(label.clone(), instructions.len());
                }
                Ok(instruction) => {
                    instructions.push((line_number, instruction.clone()));
                }
                _ => {}
            }
        }

        let possible_parse_error = Self::parse_error(&instruction_parse_results, &marks);

        let program = Program {
            file_path: String::new(),
            raw_lines: instruction_strings
                .iter()
                .map(ToString::to_string)
                .collect(),
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
    ///
    /// # Panics
    ///
    /// If the given file path is not a *.exa file.
    pub fn new_from_file(file_name: &str) -> Result<Self, ParseError> {
        assert!(
            Self::has_exa_extension(file_name),
            "File {file_name} is invalid, and must end with '.exa'"
        );

        Program::new(&to_string_vector(file_name).unwrap()).map(|mut program| {
            program.file_path = file_name.to_string();

            program
        })
    }

    /// Returns the line number and [`Instruction`] tuple at the current stack index.
    ///
    /// If the stack index is not in the instructions map, then return [`Empty`];
    pub fn peak_current_instruction(&self) -> Option<(usize, Instruction)> {
        let result = self.instructions.get(self.stack_index).cloned();

        result
    }

    /// Returns the line number and [`Instruction`] tuple at the current stack index.
    ///
    /// If the stack index is not in the instructions map, then return [`Empty`];
    ///
    /// This will increase the stack by 1.
    pub fn get_current_instruction(&mut self) -> Option<(usize, Instruction)> {
        let result = self.instructions.get(self.stack_index).cloned();

        if result.is_some() {
            self.stack_index += 1;
        }

        result
    }

    /// Sets the stack index the respective `MARK` [`Value`].
    ///
    /// A MARK identifies a line number to set the index to. However, since there can be comments,
    /// notes, empty line, or even other marks, this will find the next instruction after the mark.
    ///
    /// # Panics
    ///
    /// If the given label is not in the list of `MARKS` or if the given [`Value`] is not
    /// a [`Value::LabelId`].
    pub fn jump_to(&mut self, mark_label: &Value) {
        self.stack_index = match mark_label {
            Value::LabelId(label) => self
                .marks
                .get(label)
                .copied()
                .unwrap_or_else(|| panic!("{label} is not a valid MARK!")),
            _ => panic!("{mark_label:?} is not a Value::LabelId!"),
        };
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

        errors.sort_by_key(LineParseError::line_number);

        if errors.is_empty() {
            None
        } else {
            Some(ParseError(errors))
        }
    }

    /// Indicates if the provide file name has the ".exa" extension.
    fn has_exa_extension(file_name: &str) -> bool {
        Path::new(file_name)
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("exa"))
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

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw_lines.join("\n"))
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

        let expected_instructions = [
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
            raw_lines: instructions.clone(),
            instructions: expected_instructions
                .iter()
                .enumerate()
                .filter(|(_, s)| !s.is_empty())
                .filter_map(|(i, s)| s.parse().map(|instruction| (i, instruction)).ok())
                .collect(),
            marks: HashMap::from([(String::from("THIS_LABEL"), 1)]),
            stack_index: 0,
        };

        let program = Program::try_from(instructions.as_slice());

        assert_eq!(program, Ok(expected_program));
    }

    #[test]
    fn test_try_from_str_array_err() {
        let instructions = [
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
        let expected_raw_lines = vec![
            String::from("LINK 800"),
            String::new(),
            String::from("COPY 4 X"),
            String::new(),
            String::from("# Loop a few times"),
            String::from("MARK THIS_LABEL"),
            String::from("SUBI X 1 X"),
            String::from("TEST X = 0"),
            String::from("FJMP THIS_LABEL"),
            String::new(),
            String::from("HALT"),
        ];

        let expected_instructions = vec![
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

        let expected_program = Program {
            file_path: String::from("test_files/simple_program.exa"),
            raw_lines: expected_raw_lines,
            instructions: expected_instructions,
            marks: HashMap::from([(String::from("THIS_LABEL"), 2)]),
            stack_index: 0,
        };

        let program = Program::new_from_file("test_files/simple_program.exa");

        assert_eq!(program, Ok(expected_program));
    }

    #[test]
    fn test_get_current_instruction() {
        let mut program = Program::try_from([
            "LINK 800",
            "",
            "COPY 4 X",
            "",
            "# Loop a few times",
            "MARK THIS_LABEL",
            "SUBI X 1 X",
            "TEST X = 0",
            "FJMP THIS_LABEL",
            "",
            "HALT",
        ])
        .unwrap();

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

        while let Some(instruction) = program.get_current_instruction() {
            results.push(instruction);
        }

        assert!(program.get_current_instruction().is_none());
        assert_eq!(program.stack_index, 6);
        assert_eq!(results, expected);
    }

    #[test]
    fn test_jump_to() {
        let mut program = Program::try_from([
            "LINK 800",
            "",
            "COPY 4 X",
            "COPY 4 X",
            "",
            "# Loop a few times",
            "MARK THIS_LABEL",
            "MARK ANOTHER_LABEL",
            "NOTE skip this ",
            "# skip this too",
            "",
            "SUBI X 1 X",
            "TEST X = 0",
            "FJMP THIS_LABEL",
            "",
            "HALT",
        ])
        .unwrap();

        program.jump_to(&Value::LabelId(String::from("ANOTHER_LABEL")));

        assert_eq!(program.stack_index, 3);
    }
}
