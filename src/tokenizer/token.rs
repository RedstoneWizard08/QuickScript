use crate::{throw, tokenizer::error::Error};

use super::{cursor::Cursor, data::TokenData, operator::Operator, punct::Punct};
use std::ops::Range;

pub const SYMBOLS: &[char] = &[
    '=', '>', '<', '!', '+', '-', '*', '/', '%', '&', '|', '^', '~', '(', ')', '{', '}', '[', ']',
    ';', ':', ',', '.', '"', '\'',
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    pub content: TokenData,
    pub position: Range<usize>,
}

impl Token {
    pub fn read(buf: &mut Cursor) -> Self {
        let val = buf.next();
        let start = buf.position - 1;

        if val.is_none() {
            return Self {
                content: TokenData::EndOfFile,
                position: Range {
                    start,
                    end: buf.position,
                },
            };
        }

        match val.unwrap() {
            '=' => {
                if buf.peek() == Some('=') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::Equal),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else {
                    Self {
                        content: TokenData::Operator(Operator::Assign),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }

            '>' => {
                if buf.peek() == Some('=') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::GreaterEqual),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else {
                    Self {
                        content: TokenData::Operator(Operator::Greater),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }

            '<' => {
                if buf.peek() == Some('=') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::LessEqual),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else {
                    Self {
                        content: TokenData::Operator(Operator::Less),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }

            '!' => {
                if buf.peek() == Some('=') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::NotEqual),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else {
                    Self {
                        content: TokenData::Operator(Operator::Not),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }

            '+' => {
                if buf.peek() == Some('=') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::AddAssign),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else {
                    Self {
                        content: TokenData::Operator(Operator::Add),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }

            '-' => {
                if buf.peek() == Some('=') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::SubAssign),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else {
                    Self {
                        content: TokenData::Operator(Operator::Sub),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }

            '*' => {
                if buf.peek() == Some('=') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::MulAssign),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else {
                    Self {
                        content: TokenData::Operator(Operator::Mul),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }

            '/' => {
                if buf.peek() == Some('=') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::DivAssign),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else {
                    Self {
                        content: TokenData::Operator(Operator::Div),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }

            '%' => {
                if buf.peek() == Some('=') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::ModAssign),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else {
                    Self {
                        content: TokenData::Operator(Operator::Mod),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }

            '&' => {
                if buf.peek() == Some('=') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::BitwiseAndAssign),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else if buf.peek() == Some('&') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::And),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else {
                    Self {
                        content: TokenData::Operator(Operator::BitwiseAnd),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }

            '|' => {
                if buf.peek() == Some('=') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::BitwiseOrAssign),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else if buf.peek() == Some('|') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::Or),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else {
                    Self {
                        content: TokenData::Operator(Operator::BitwiseOr),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }

            '^' => {
                if buf.peek() == Some('=') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::XorAssign),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else {
                    Self {
                        content: TokenData::Operator(Operator::Xor),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }

            '~' => {
                if buf.peek() == Some('=') {
                    buf.next();

                    Self {
                        content: TokenData::Operator(Operator::BitwiseNotAssign),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else {
                    Self {
                        content: TokenData::Operator(Operator::BitwiseNot),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }

            '(' => Self {
                content: TokenData::Punct(Punct::OpenParen),
                position: Range {
                    start,
                    end: buf.position,
                },
            },

            ')' => Self {
                content: TokenData::Punct(Punct::CloseParen),
                position: Range {
                    start,
                    end: buf.position,
                },
            },

            '{' => Self {
                content: TokenData::Punct(Punct::OpenBrace),
                position: Range {
                    start,
                    end: buf.position,
                },
            },

            '}' => Self {
                content: TokenData::Punct(Punct::CloseBrace),
                position: Range {
                    start,
                    end: buf.position,
                },
            },

            '[' => Self {
                content: TokenData::Punct(Punct::OpenBracket),
                position: Range {
                    start,
                    end: buf.position,
                },
            },

            ']' => Self {
                content: TokenData::Punct(Punct::CloseBracket),
                position: Range {
                    start,
                    end: buf.position,
                },
            },

            ';' => Self {
                content: TokenData::Punct(Punct::Semicolon),
                position: Range {
                    start,
                    end: buf.position,
                },
            },

            ':' => Self {
                content: TokenData::Punct(Punct::Colon),
                position: Range {
                    start,
                    end: buf.position,
                },
            },

            ',' => Self {
                content: TokenData::Punct(Punct::Comma),
                position: Range {
                    start,
                    end: buf.position,
                },
            },

            '.' => {
                if buf.peek() == Some('.') {
                    buf.next();

                    Self {
                        content: TokenData::Punct(Punct::DotDot),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                } else {
                    Self {
                        content: TokenData::Punct(Punct::Dot),
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }

            '"' => {
                let mut content = String::new();

                loop {
                    let c = buf.next();

                    if c == Some('"') {
                        break;
                    }

                    if c == None {
                        let err = Error::Expected {
                            expected: "'\"'".to_string(),
                            file: buf.file.clone(),
                            data: buf.all_data.clone(),
                            pos: Range {
                                start,
                                end: buf.position,
                            },
                        };

                        throw!(err);
                    }

                    content.push(c.unwrap());
                }

                let content = content
                    .replace("\\n", "\n")
                    .replace("\\r", "\r")
                    .replace("\\t", "\t")
                    .replace("\\0", "\0");

                Self {
                    content: TokenData::String(content),
                    position: Range {
                        start,
                        end: buf.position,
                    },
                }
            }

            '\'' => {
                let mut content = String::new();

                loop {
                    let c = buf.next();

                    if c == Some('\'') {
                        break;
                    }

                    if c == None {
                        let err = Error::Expected {
                            expected: "\"'\"".to_string(),
                            file: buf.file.clone(),
                            data: buf.all_data.clone(),
                            pos: Range {
                                start,
                                end: buf.position,
                            },
                        };

                        throw!(err);
                    }

                    content.push(c.unwrap());
                }

                let position = Range {
                    start,
                    end: buf.position,
                };

                match content.as_str() {
                    "\\n" => Self {
                        content: TokenData::Char('\n'),
                        position,
                    },

                    "\\r" => Self {
                        content: TokenData::Char('\r'),
                        position,
                    },

                    "\\t" => Self {
                        content: TokenData::Char('\t'),
                        position,
                    },

                    "\\0" => Self {
                        content: TokenData::Char('\0'),
                        position,
                    },

                    _ => Self {
                        content: TokenData::Char(content.chars().next().unwrap()),
                        position,
                    },
                }
            }

            t => {
                if t.is_alphabetic() {
                    let mut content = String::new();

                    content.push(t);

                    loop {
                        let c = buf.peek();

                        if c.is_none() {
                            break;
                        }

                        if c.unwrap().is_alphanumeric() || c.unwrap() == '_' {
                            content.push(buf.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    match content.as_str() {
                        "true" => Self {
                            content: TokenData::Boolean(true),
                            position: Range {
                                start,
                                end: buf.position,
                            },
                        },

                        "false" => Self {
                            content: TokenData::Boolean(false),
                            position: Range {
                                start,
                                end: buf.position,
                            },
                        },

                        _ => Self {
                            content: TokenData::Name(content),
                            position: Range {
                                start,
                                end: buf.position,
                            },
                        },
                    }
                } else if t.is_numeric() {
                    let mut content = String::new();

                    content.push(t);

                    loop {
                        let c = buf.peek();

                        if c.is_none() {
                            break;
                        }

                        if c.unwrap().is_numeric() {
                            content.push(buf.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    match content.parse::<i32>() {
                        Ok(num) => Self {
                            content: TokenData::Number(num),
                            position: Range {
                                start,
                                end: buf.position,
                            },
                        },

                        Err(_) => {
                            let err = Error::UnexpectedToken {
                                token: content,
                                file: buf.file.clone(),
                                data: buf.all_data.clone(),
                                pos: Range {
                                    start,
                                    end: buf.position,
                                },
                            };

                            throw!(err);
                        }
                    }
                } else {
                    Self {
                        content: TokenData::None,
                        position: Range {
                            start,
                            end: buf.position,
                        },
                    }
                }
            }
        }
    }
}
