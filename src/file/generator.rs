use std::cell::RefCell;
use std::rc::{Rc, Weak};

use super::id_generator::IdGenerator;
use super::File;

/// The file generator holds a reference counted pointer to an [`IdGenerator`]
/// so that every Exa can generate files without conflicting Ids.
#[derive(Debug, Clone)]
pub struct Generator {
    id_generator: Weak<RefCell<IdGenerator>>,
}

impl Generator {
    /// Creates a new [`Generator`] with a given reference counted pointer to
    /// an [`IdGenerator`].
    pub fn new(id_generator: &Rc<RefCell<IdGenerator>>) -> Self {
        Generator {
            id_generator: Rc::downgrade(id_generator),
        }
    }

    /// Generates a new [`File`] object with a generated id and no contents.
    ///
    /// # Panics
    ///
    /// If the generated id is greater than 9999.
    #[must_use]
    pub fn generate(&self) -> File {
        File::new(
            &self
                .id_generator
                .upgrade()
                .unwrap()
                .borrow_mut()
                .next()
                .unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_files_with_multiple_generators() {
        let id_generator = Rc::new(RefCell::new(IdGenerator::default()));

        let file_generator_1 = Generator::new(&id_generator);
        let file_generator_2 = Generator::new(&id_generator);

        let result_1 = file_generator_1.generate();
        let result_2 = file_generator_2.generate();
        let result_3 = file_generator_1.generate();
        let result_4 = file_generator_2.generate();

        assert_eq!(result_1.id, "400");
        assert_eq!(result_2.id, "401");
        assert_eq!(result_3.id, "402");
        assert_eq!(result_4.id, "403");
    }
}
