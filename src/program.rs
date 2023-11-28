use std::collections::HashMap;

use crate::instruction::Instruction;

/// A Program is just a stack of [`Instruction`]s.
///
/// It can jump to certain instructions, it can return the next instruction in the stack, and keeps
/// track of instruction index.
///
/// These can be created manually or via a customer *.exa file type.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Program {
    file_path: String,
    instructions: Vec<(usize, Instruction)>,
    marks: HashMap<String, usize>,
    stack_index: usize,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
        unimplemented!()
    }
}
