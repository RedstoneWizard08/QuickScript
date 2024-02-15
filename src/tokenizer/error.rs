use std::ops::Range;
use thiserror::Error;

pub fn format_position(file: &String, data: &String, pos: &Range<usize>) -> String {
    let mut lines = 0;
    let mut chars = 0;

    for (i, ch) in data.chars().enumerate() {
        if i >= pos.start {
            break;
        }

        if ch == '\n' {
            lines += 1;
            chars = 0;
        } else {
            chars += 1;
        }
    }

    format!("=> at {}:{}:{}", file, lines, chars + 4)
}

#[derive(Debug, Clone, Error, PartialEq, Eq, Hash)]
pub enum Error {
    #[error("Unexpected character: {}\n{}", .ch, format_position(.file, .data, .pos))]
    UnexpectedChar {
        ch: char,
        file: String,
        data: String,
        pos: Range<usize>,
    },

    #[error("Unexpected end of file\n{}", format_position(.file, .data, .pos))]
    UnexpectedEof {
        file: String,
        data: String,
        pos: Range<usize>,
    },

    #[error("Unexpected token: {}\n{}", .token, format_position(.file, .data, .pos))]
    UnexpectedToken {
        token: String,
        file: String,
        data: String,
        pos: Range<usize>,
    },

    #[error("Expected {}\n{}", .expected, format_position(.file, .data, .pos))]
    Expected {
        expected: String,
        file: String,
        data: String,
        pos: Range<usize>,
    },

    #[error("Unknown error\n{}", format_position(.file, .data, .pos))]
    Unknown {
        file: String,
        data: String,
        pos: Range<usize>,
    },
}
