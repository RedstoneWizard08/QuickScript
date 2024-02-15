#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cursor {
    pub file: String,
    pub all_data: String,
    pub data: Vec<char>,
    pub position: usize,
}

impl Cursor {
    pub fn new(file: String, data: Vec<char>) -> Self {
        Self {
            data: data.clone(),
            file,
            all_data: String::from_iter(data),
            position: 0,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if self.data.is_empty() {
            return None;
        }

        self.position += 1;

        Some(self.data.remove(0))
    }

    pub fn peek(&self) -> Option<char> {
        self.data.first().cloned()
    }

    pub fn peek_at(&self, index: usize) -> Option<char> {
        self.data.get(index).cloned()
    }
}
