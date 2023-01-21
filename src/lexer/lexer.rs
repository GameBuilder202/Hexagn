use std::process::exit;

use crate::util::print_error;
use crate::unwrap_or_err;
use crate::util::find_nth;

#[derive(Debug, Clone)]
pub struct Token {
    pub lineno: usize,
    pub tok_type: TokenType,
    pub val: String,

    pub start: usize,
    pub end: usize,
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    // Builtin datatypes
    Void,
    Int,
    Uint,
    Float,
    String,
    Character,

    // Other stuff
    Identifier,
    Assign,
    Num,
    Flt,
    Str,
    Char,

    OpenParen,
    CloseParen,
    Comma,

    Semicolon,

    Plus,
    Minus,
    Mult,
    Div,
    Mod,

    OpenBrace,
    CloseBrace,

    If,
    Else,
    While,

    EQ,
    NEQ,
    GT,
    GTE,
    LT,
    LTE,

    Import,
    Dot,
    Colon,

    URCLBlock,

    Return,

    Extern,
    Pub,
}

const SIGNED_INT_TYPES: [&str; 4] = ["int8", "int16", "int32", "int64"];

const UNSIGNED_INT_TYPES: [&str; 4] = ["uint8", "uint16", "uint32", "uint64"];

const FLOAT_TYPES: [&str; 2] = ["float32", "float64"];

pub fn tokenize(src: &String) -> Vec<Token> {
    let mut res = Vec::new();
    let mut buf = Buffer::new(src);
    let mut lineno = 0;

    while buf.in_bounds() {
        let data = buf.current("", &Default::default());

        if data == ' ' {
            buf.advance();
            continue;
        } else if data == '\n' {
            lineno += 1;
        } else if data == ';' {
            res.push(Token {
                lineno,
                tok_type: TokenType::Semicolon,
                val: ";".to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == '=' {
            let start = buf.line_pos(&lineno);
            buf.advance();
            if buf.in_bounds() && buf.current("", &Default::default()) == '=' {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::EQ,
                    val: "==".to_string(),

                    start,
                    end: buf.line_pos(&lineno),
                })
            } else {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::Assign,
                    val: data.to_string(),

                    start: buf.line_pos(&lineno),
                    end: buf.line_pos(&lineno),
                })
            }
        } else if data == '+' {
            res.push(Token {
                lineno,
                tok_type: TokenType::Plus,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == '-' {
            res.push(Token {
                lineno,
                tok_type: TokenType::Minus,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == '*' {
            res.push(Token {
                lineno,
                tok_type: TokenType::Mult,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == '/' {
            res.push(Token {
                lineno,
                tok_type: TokenType::Div,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == '%' {
            res.push(Token {
                lineno,
                tok_type: TokenType::Mod,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == '+' {
            res.push(Token {
                lineno,
                tok_type: TokenType::Plus,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == '-' {
            res.push(Token {
                lineno,
                tok_type: TokenType::Minus,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == '*' {
            res.push(Token {
                lineno,
                tok_type: TokenType::Mult,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == '/' {
            res.push(Token {
                lineno,
                tok_type: TokenType::Div,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == '(' {
            res.push(Token {
                lineno,
                tok_type: TokenType::OpenParen,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == ')' {
            res.push(Token {
                lineno,
                tok_type: TokenType::CloseParen,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == ',' {
            res.push(Token {
                lineno,
                tok_type: TokenType::Comma,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == '{' {
            res.push(Token {
                lineno,
                tok_type: TokenType::OpenBrace,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == '}' {
            res.push(Token {
                lineno,
                tok_type: TokenType::CloseBrace,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == '>' {
            let start = buf.line_pos(&lineno);
            buf.advance();
            if buf.in_bounds() && buf.current("", &Default::default()) == '=' {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::GTE,
                    val: ">=".to_string(),

                    start,
                    end: buf.line_pos(&lineno),
                })
            } else {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::GT,
                    val: data.to_string(),

                    start: buf.line_pos(&lineno),
                    end: buf.line_pos(&lineno),
                })
            }
        } else if data == '<' {
            let start = buf.line_pos(&lineno);
            buf.advance();
            if buf.in_bounds() && buf.current("", &Default::default()) == '=' {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::LTE,
                    val: "<=".to_string(),

                    start,
                    end: buf.line_pos(&lineno),
                })
            } else {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::LT,
                    val: data.to_string(),

                    start: buf.line_pos(&lineno),
                    end: buf.line_pos(&lineno),
                })
            }
        } else if data == '!' {
            let start = buf.line_pos(&lineno);
            buf.advance();
            if buf.in_bounds() && buf.current("", &Default::default()) == '=' {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::NEQ,
                    val: "!=".to_string(),

                    start,
                    end: buf.line_pos(&lineno),
                })
            } else {
                continue;
            }
        } else if data == '\'' {
            let start = buf.line_pos(&lineno);
            buf.advance();
            let _char;

            if !buf.in_bounds() {
                print_error(
                    "Expected character after ' at line {}",
                    src,
                    start,
                    buf.line_pos(&lineno),
                    lineno,
                );
                exit(1)
            }
            if buf.current("", &Default::default()) == '\\' {
                buf.advance();

                match buf.current(
                    "Expected escape character after \\",
                    &PosInfo {
                        src: src.to_string(),
                        start,
                        end: buf.line_pos(&lineno),
                        lineno,
                    },
                ) {
                    'n' => _char = '\n',
                    't' => _char = '\t',
                    '\'' => _char = '\'',
                    '\\' => _char = '\\',

                    _ => {
                        print_error(
                            "Invalid escape character at line {}",
                            src,
                            start,
                            buf.line_pos(&lineno),
                            lineno,
                        );
                        exit(1)
                    }
                }
            } else {
                _char = buf.current("", &Default::default())
            }

            if buf.next(
                "Expected closing ' for character literal",
                &PosInfo {
                    src: src.to_string(),
                    start,
                    end: buf.line_pos(&lineno),
                    lineno,
                },
            ) != '\''
            {
                print_error(
                    "Expected closing ' at line {}",
                    src,
                    start,
                    buf.line_pos(&lineno),
                    lineno,
                );
                exit(1)
            }

            res.push(Token {
                lineno,
                tok_type: TokenType::Num,
                val: (_char as u8).to_string(),

                start,
                end: buf.line_pos(&lineno),
            })
        } else if data == '"' {
            let start = buf.line_pos(&lineno);
            buf.advance();
            let mut _str = String::new();

            while buf.current("", &Default::default()) != '"' {
                if buf.current("", &Default::default()) == '\\' {
                    buf.advance();

                    match buf.current(
                        "Expected escape character after \\",
                        &PosInfo {
                            src: src.to_string(),
                            start,
                            end: buf.line_pos(&lineno),
                            lineno,
                        },
                    ) {
                        'n' => _str += "\n",
                        't' => _str += "\t",
                        '\'' => _str += "\'",
                        '\\' => _str += "\\",

                        _ => {
                            print_error(
                                "Invalid escape character at line {}",
                                src,
                                start,
                                buf.line_pos(&lineno),
                                lineno,
                            );
                            exit(1)
                        }
                    }
                } else if buf.current("", &Default::default()) == '\n' {
                    print_error(
                        "Unterminated string at line {}",
                        src,
                        start,
                        buf.line_pos(&lineno),
                        lineno,
                    );
                    exit(1)
                } else {
                    _str += &buf.current("", &Default::default()).to_string()
                }

                buf.advance()
            }

            res.push(Token {
                lineno,
                tok_type: TokenType::Str,
                val: _str,

                start,
                end: buf.line_pos(&lineno),
            })
        } else if data == '.' {
            res.push(Token {
                lineno,
                tok_type: TokenType::Dot,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data == ':' {
            res.push(Token {
                lineno,
                tok_type: TokenType::Colon,
                val: data.to_string(),

                start: buf.line_pos(&lineno),
                end: buf.line_pos(&lineno),
            })
        } else if data.is_alphabetic() || data == '_' {
            let mut word = String::new();

            let start = buf.line_pos(&lineno);

            while buf.in_bounds() {
                let curr = buf.current("", &Default::default());

                if curr == ' ' || (!curr.is_alphanumeric() && curr != '_') {
                    break;
                }

                word += &curr.to_string();
                buf.advance()
            }

            let end = buf.line_pos(&lineno);

            if SIGNED_INT_TYPES.contains(&word.as_str()) {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::Int,
                    val: word,

                    start,
                    end,
                })
            } else if UNSIGNED_INT_TYPES.contains(&word.as_str()) {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::Uint,
                    val: word,

                    start,
                    end,
                })
            } else if FLOAT_TYPES.contains(&word.as_str()) {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::Float,
                    val: word,

                    start,
                    end,
                })
            } else if word == "void" {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::Void,
                    val: word,

                    start,
                    end,
                })
            } else if word == "string" {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::String,
                    val: word,

                    start,
                    end,
                })
            } else if word == "char" {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::Character,
                    val: word,

                    start,
                    end,
                })
            } else if word == "if" {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::If,
                    val: word,

                    start,
                    end,
                })
            } else if word == "else" {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::Else,
                    val: word,

                    start,
                    end,
                })
            } else if word == "while" {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::While,
                    val: word,

                    start,
                    end,
                })
            } else if word == "import" {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::Import,
                    val: word,

                    start,
                    end,
                })
            } else if word == "urcl" {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::URCLBlock,
                    val: word,

                    start,
                    end,
                })
            } else if word == "return" {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::Return,
                    val: word,

                    start,
                    end,
                })
            } else if word == "extern" {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::Extern,
                    val: word,

                    start,
                    end
                })
            } else if word == "pub" {
                res.push(Token { lineno: lineno, tok_type: TokenType::Pub, val: word, start, end })
            } else if word == "return" {
                res.push(Token { lineno, tok_type: TokenType::Return, val: word, start, end })
            } else {
                res.push(Token {
                    lineno,
                    tok_type: TokenType::Identifier,
                    val: word,

                    start,
                    end,
                })
            }

            continue;
        } else if data.is_numeric() {
            let mut num = String::new();
            let start = buf.line_pos(&lineno);

            while buf.in_bounds() {
                let curr = buf.current("", &Default::default());

                if curr == ';' || !curr.is_numeric() {
                    break;
                }

                num += &curr.to_string();
                buf.advance()
            }

            let end = buf.line_pos(&lineno);

            res.push(Token {
                lineno,
                tok_type: TokenType::Num,
                val: num,

                start,
                end,
            });

            continue;
        }

        buf.advance();
    }

    res
}

struct Buffer {
    data: String,
    index: usize,
}

impl Buffer {
    pub fn new(src: &String) -> Buffer {
        Buffer {
            data: src.clone(),
            index: 0,
        }
    }

    pub fn in_bounds(&self) -> bool {
        self.index < self.data.len()
    }

    pub fn advance(&mut self) {
        self.index += 1
    }

    pub fn next(&mut self, err: &str, pos: &PosInfo) -> char {
        self.advance();
        self.current(err, pos)
    }

    pub fn current(&self, err: &str, pos: &PosInfo) -> char {
        unwrap_or_err!(
            self.data.chars().nth(self.index),
            (pos.src, pos.start, pos.end, pos.lineno, err)
        )
    }

    pub fn pos(&self) -> usize {
        self.index
    }

    pub fn line_pos(&self, lineno: &usize) -> usize {
        self.pos() - find_nth(&self.data, &'\n', &(lineno - 1))
    }
}

struct PosInfo {
    src: String,
    start: usize,
    end: usize,
    lineno: usize,
}

impl Default for PosInfo {
    fn default() -> Self {
        Self {
            src: String::new(),
            start: 0,
            end: 0,
            lineno: 0,
        }
    }
}
