use crate::value::Value;

/// A Register simply holds a [`Value`], with methods to read/write said [`Value`].
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Register {
    id: String,
    value: Option<Value>,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        unimplemented!()
    }
}
