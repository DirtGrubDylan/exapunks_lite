/// A `Value` is used to hold several types of information: number, keyword, register id, and a
/// label id. Each type is used by an [`Exa`] to perform their tasks. Whether it is storing keywords
/// from a [`Register`] to a [`File`], or asking their [`Program`] to jump to a specific label. Or
/// even adding two numbers and storing them to a [`Register`] with a specified id.
///
/// An [`Instruction`] cannot have keyword values, but they can have source/destination register ids
/// and numbers.
///
/// A [`Register`] can hold on to a number or keyword value.
#[derive(Debug)]
pub enum Value {
    Number(isize),
    Keyword(String),
    RegisterId(String),
    LabelId(String),
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_new() {
        unimplemented!()
    }

    #[test]
    fn test_add_number() {
        unimplemented!()
    }

    #[test]
    fn test_add_non_number_panics() {
        unimplemented!()
    }

    #[test]
    fn test_subtract_number() {
        unimplemented!()
    }

    #[test]
    fn test_subtract_non_number_panics() {
        unimplemented!()
    }

    #[test]
    fn test_divide_number() {
        unimplemented!()
    }

    #[test]
    fn test_divide_non_number_panics() {
        unimplemented!()
    }

    #[test]
    fn test_multiply_number() {
        unimplemented!()
    }

    #[test]
    fn test_multiply_non_number_panics() {
        unimplemented!()
    }

    #[test]
    fn test_modulo_number() {
        unimplemented!()
    }

    #[test]
    fn test_modulo_non_number_panics() {
        unimplemented!()
    }

    #[test]
    fn test_greater_than_number() {
        unimplemented!()
    }

    #[test]
    fn test_greater_than_keyword() {
        unimplemented!()
    }

    #[test]
    fn test_greater_than_non_number_non_keyword_panics() {
        unimplemented!()
    }

    #[test]
    fn test_less_than_number() {
        unimplemented!()
    }

    #[test]
    fn test_less_than_keyword() {
        unimplemented!()
    }

    #[test]
    fn test_less_than_non_number_non_keyword_panics() {
        unimplemented!()
    }

    #[test]
    fn test_equal_to_number() {
        unimplemented!()
    }

    #[test]
    fn test_equal_to_keyword() {
        unimplemented!()
    }

    #[test]
    fn test_equal_to_non_number_non_keyword_panics() {
        unimplemented!()
    }

    #[test]
    fn test_from_str_to_number() {
        unimplemented!()
    }

    #[test]
    fn test_from_str_to_keyword() {
        unimplemented!()
    }

    #[test]
    fn test_from_isize_to_number() {
        unimplemented!()
    }

    #[test]
    fn test_from_number_to_isize() {
        unimplemented!()
    }

    #[test]
    fn test_from_non_number_to_isize_panics() {
        unimplemented!()
    }

    #[test]
    fn test_from_number_to_string() {
        unimplemented!()
    }

    #[test]
    fn test_from_non_number_to_string() {
        unimplemented!()
    }
}
