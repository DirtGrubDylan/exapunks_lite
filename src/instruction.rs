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

