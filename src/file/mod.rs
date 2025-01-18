pub mod generator;
pub mod id_generator;

use crate::value::Value;

/// A File holds an identifier, a list of [`Value`]s, and an index.
///
/// The values are either [`Value::Number`] or [`Value::Keyword`].
#[derive(Debug, PartialEq, Clone)]
pub struct File {
    pub id: String,
    contents: Vec<Value>,
    index: usize,
}

impl File {
    /// Creates a new file, with a given id, with no contents.
    #[must_use]
    pub fn new(id: &str) -> Self {
        File {
            id: id.to_string(),
            contents: Vec::new(),
            index: 0,
        }
    }

    /// Creates a new file with a given id and contents.
    ///
    /// The contents are parsed to [`Value`]s, ignoring any parse errors.
    /// This effectively will only skip empty lines.
    #[must_use]
    pub fn new_with_contents(id: &str, contents: &[String]) -> Self {
        File {
            id: id.to_string(),
            contents: contents
                .iter()
                .filter_map(|line| line.parse().ok())
                .collect(),
            index: 0,
        }
    }

    /// Returns the id as a [`Value`].
    #[must_use]
    pub fn id(&self) -> Value {
        Value::from(self.id.as_str())
    }

    /// Returns the number of contents in the file.
    #[must_use]
    pub fn len(&self) -> usize {
        self.contents.len()
    }

    /// Indicates if the file is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.contents.is_empty()
    }

    /// Returns the possible [`Value`] at the file's internal index.
    #[must_use]
    pub fn current(&self) -> Option<Value> {
        self.contents.get(self.index).cloned()
    }

    /// Indicates if the file's index is equal to the length of its contents.
    #[must_use]
    pub fn is_eof(&self) -> bool {
        self.index == self.len()
    }

    /// Adjusts the file's index by a given offset.
    ///
    /// The index will be bound by: [0, {length of contents}].
    pub fn adjust_index(&mut self, offset: isize) {
        self.index = self.len().min(self.index.saturating_add_signed(offset));
    }

    /// Appends the given [`Value`] to the file's contents.
    pub fn append(&mut self, with: &Value) {
        self.contents.push(with.clone());
    }

    /// Removes the item at the file's index.
    ///
    /// If the index is equal to the length of it's contents, then do nothing.
    pub fn remove_current(&mut self) {
        if !self.is_eof() {
            self.contents.remove(self.index);
        }
    }

    /// Replaces the item at the file's index with the given [`Value`].
    ///
    /// If the index is equal to the length of it's contents, then append.
    pub fn replace_current(&mut self, with: &Value) {
        if self.is_eof() {
            self.append(with);
        } else {
            self.remove_current();
            self.contents.insert(self.index, with.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_index_by_positive_2() {
        let contents = [
            String::from("keyword1"),
            String::from("666"),
            String::from("keyword2"),
            String::from("333"),
            String::from("keyword3"),
        ];

        let mut file = File::new_with_contents("id", &contents);

        file.adjust_index(2);

        assert_eq!(file.index, 2);
    }

    #[test]
    fn test_adjust_index_by_positive_9999_max_capped() {
        let contents = [
            String::from("keyword1"),
            String::from("666"),
            String::from("keyword2"),
            String::from("333"),
            String::from("keyword3"),
        ];

        let mut file = File::new_with_contents("id", &contents);

        file.adjust_index(9_999);

        assert_eq!(file.index, 5);
    }

    #[test]
    fn test_adjust_index_by_negative_1() {
        let contents = [
            String::from("keyword1"),
            String::from("666"),
            String::from("keyword2"),
            String::from("333"),
            String::from("keyword3"),
        ];

        let mut file = File::new_with_contents("id", &contents);

        file.adjust_index(2);
        file.adjust_index(-1);

        assert_eq!(file.index, 1);
    }

    #[test]
    fn test_adjust_index_by_negative_9999_min_capped() {
        let contents = [
            String::from("keyword1"),
            String::from("666"),
            String::from("keyword2"),
            String::from("333"),
            String::from("keyword3"),
        ];

        let mut file = File::new_with_contents("id", &contents);

        file.adjust_index(2);
        file.adjust_index(-9_999);

        assert_eq!(file.index, 0);
    }

    #[test]
    fn test_append() {
        let contents = [
            String::from("keyword1"),
            String::from("666"),
            String::from("keyword2"),
            String::from("333"),
        ];

        let mut file = File::new_with_contents("id", &contents);

        let expected_contents = [
            Value::from("keyword1"),
            Value::from("666"),
            Value::from("keyword2"),
            Value::from("333"),
            Value::from("appending"),
        ];

        file.append(&Value::from("appending"));

        assert_eq!(file.index, 0);
        assert_eq!(file.contents, expected_contents);
    }

    #[test]
    fn test_remove_current_from_middle() {
        let contents = [
            String::from("keyword1"),
            String::from("666"),
            String::from("keyword2"),
            String::from("333"),
            String::from("keyword3"),
        ];

        let mut file = File::new_with_contents("id", &contents);

        let expected_contents = [
            Value::from("keyword1"),
            Value::from("666"),
            Value::from("333"),
            Value::from("keyword3"),
        ];

        file.adjust_index(2);
        file.remove_current();

        assert_eq!(file.index, 2);
        assert_eq!(file.contents, expected_contents);
    }

    #[test]
    fn test_remove_current_from_last_item_index() {
        let contents = [
            String::from("keyword1"),
            String::from("666"),
            String::from("keyword2"),
            String::from("333"),
            String::from("keyword3"),
        ];

        let mut file = File::new_with_contents("id", &contents);

        let expected_contents = [
            Value::from("keyword1"),
            Value::from("666"),
            Value::from("keyword2"),
            Value::from("333"),
        ];

        file.adjust_index(4);
        file.remove_current();

        assert_eq!(file.index, 4);
        assert_eq!(file.contents, expected_contents);
    }

    #[test]
    fn test_replace_current_from_middle() {
        let contents = [
            String::from("keyword1"),
            String::from("666"),
            String::from("keyword2"),
            String::from("333"),
            String::from("keyword3"),
        ];

        let mut file = File::new_with_contents("id", &contents);

        let expected_contents = [
            Value::from("keyword1"),
            Value::from("666"),
            Value::from("000"),
            Value::from("333"),
            Value::from("keyword3"),
        ];

        file.adjust_index(2);
        file.replace_current(&Value::from("000"));

        assert_eq!(file.index, 2);
        assert_eq!(file.contents, expected_contents);
    }

    #[test]
    fn test_replace_current_from_eof() {
        let contents = [
            String::from("keyword1"),
            String::from("666"),
            String::from("keyword2"),
            String::from("333"),
            String::from("keyword3"),
        ];

        let mut file = File::new_with_contents("id", &contents);

        let expected_contents = [
            Value::from("keyword1"),
            Value::from("666"),
            Value::from("keyword2"),
            Value::from("333"),
            Value::from("keyword3"),
            Value::from("000"),
        ];

        file.adjust_index(9_999);
        file.replace_current(&Value::from("000"));

        assert_eq!(file.index, 5);
        assert_eq!(file.contents, expected_contents);
    }
}
