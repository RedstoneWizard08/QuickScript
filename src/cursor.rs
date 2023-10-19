use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A simple trait to depict something that can read
/// from an unspecified buffer.
pub trait Reader<T> {
    /// Reads a character or byte from the buffer.
    fn read(&mut self) -> T;
}

/// A Cursor wraps a buffer for ease of reading, while
/// tracking the current byte position.
///
/// All properties are public and can be modified in
/// real-time. Just make sure to not change the
/// content without resetting or changing the position,
/// or you'll get an index that is out of bounds!
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor<T>
where
    T: AsRef<[u8]>,
{
    pub content: T,
    pub position: usize,
    pub lines_cols: HashMap<usize, (usize, usize)>,
}

impl<T> Cursor<T>
where
    T: AsRef<[u8]>,
{
    /// Creates a new cursor. Takes in the content, which
    /// will be stored for reading. The default position
    /// is set to 0.
    pub fn new(content: T) -> Self {
        Self {
            content,
            position: 0,
            lines_cols: HashMap::new(),
        }
    }

    /// Checks whether or not there is another byte or
    /// character left to read. Returns true if there is,
    /// and false if there isn't.
    pub fn has_next(&self) -> bool {
        self.position < self.content.as_ref().len()
    }

    /// Skips [size] indexes. Returns the position after
    /// skipping.
    pub fn skip(&mut self, size: usize) -> usize {
        self.position += size;
        self.position
    }
}

impl Cursor<String> {
    pub fn from_string(content: String) -> Self {
        let mut lines_cols = HashMap::new();
        let mut pos = 0;

        for (idx, line) in content.split("\n").enumerate() {
            for (ch_idx, _) in line.chars().enumerate() {
                let ch_pos = (idx + 1, ch_idx + 1);

                lines_cols.insert(pos + ch_idx, ch_pos);
            }

            pos += line.len();
        }

        Self {
            content,
            position: 0,
            lines_cols,
        }
    }

    pub fn get_pos(&self) -> (usize, usize) {
        self.lines_cols[&self.position]
    }
}

impl<T> Reader<u8> for Cursor<T>
where
    T: AsRef<[u8]>,
{
    /// Reads a byte from the buffer. This will advance the
    /// position value.
    fn read(&mut self) -> u8 {
        let res = self.content.as_ref().get(self.position).unwrap();

        self.position += 1;

        *res
    }
}

impl Reader<char> for Cursor<String> {
    /// Reads a character from the buffer. This will advance
    /// the position value.
    fn read(&mut self) -> char {
        let res = self.content.chars().nth(self.position).unwrap();

        self.position += 1;

        res
    }
}
