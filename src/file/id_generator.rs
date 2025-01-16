use std::collections::HashSet;

/// The File ID Generator will generate a [`String`] based on
/// an incrementing integer. It aslo has a set of ids to avoid
/// generating and will panic if the id is greater than 9999.
#[derive(Debug, PartialEq, Clone)]
pub struct IdGenerator {
    next_id: usize,
    ids_to_avoid: HashSet<usize>,
}

impl IdGenerator {
    /// Creates a new `IdGenerator` with a given list of integers to avoid.
    #[must_use]
    pub fn new(ids_to_avoid_list: &[usize]) -> Self {
        let mut next_id = 400;
        let ids_to_avoid: HashSet<usize> = ids_to_avoid_list.iter().copied().collect();

        while ids_to_avoid.contains(&next_id) {
            next_id += 1;
        }

        IdGenerator {
            next_id,
            ids_to_avoid,
        }
    }
}

impl Default for IdGenerator {
    fn default() -> Self {
        IdGenerator {
            next_id: 400,
            ids_to_avoid: HashSet::new(),
        }
    }
}

impl Iterator for IdGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        assert!(
            self.next_id <= 9_999,
            "IdGenerator exceeded the maximum amount of ids (9999)!"
        );

        let result = self.next_id.to_string();

        self.next_id += 1;

        while self.ids_to_avoid.contains(&self.next_id) {
            self.next_id += 1;
        }

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut id_generator = IdGenerator::default();

        assert_eq!(id_generator.next(), Some(String::from("400")));
        assert_eq!(id_generator.next(), Some(String::from("401")));
    }

    #[test]
    fn test_next_avoiding_400_402_and_403() {
        let mut id_generator = IdGenerator::new(&[400, 402, 403]);

        assert_eq!(id_generator.next(), Some(String::from("401")));
        assert_eq!(id_generator.next(), Some(String::from("404")));
        assert_eq!(id_generator.next(), Some(String::from("405")));
    }

    #[test]
    #[should_panic(expected = "IdGenerator exceeded the maximum amount of ids (9999)!")]
    fn test_next_panics_over_9999() {
        let id_generator = IdGenerator::default();

        let mut iter = id_generator.skip(9_599);

        assert_eq!(iter.next(), Some(String::from("9999")));

        iter.next();
    }
}
