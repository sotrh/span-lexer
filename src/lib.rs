use std::{cmp, fmt};

// TODO: support unicode 🐶🚀

mod span;

pub struct Lexer<'a> {
    input: &'a str,
    last_span: Span,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let last_span = Span {
            line_num: 0,
            start: 0,
            first_byte: 0,
            last_byte: None,
        };
        Self { input, last_span }
    }

    // TODO implement iterator
    pub fn next(&mut self) -> Option<(Token<'a>, Span)> {
        let mut chars = self.input.char_indices();
        let mut span = self.last_span;

        while let Some((b, c)) = chars.next() {
            match c {
                ' ' | '\t' | '\n' => {
                    if c == '\n' {
                        span.start = 0;
                        span.line_num += 1;
                    } else {
                        span.start += 1;
                    }
                    span.first_byte = b;
                    span.last_byte = None;
                }
                // TODO: support floats
                '0'..='9' => {
                    span.first_byte = b;
                    span.last_byte = Some(b);
                    let input = chars.as_str();
                    // TODO: is this union needed?
                    span = span.union(Self::consume_number(input, span));
                }
                '+' => {
                    span.first_byte = b;
                    span.last_byte = Some(b);
                }
                _ => todo!(),
            }
        }
        
        self.input = chars.as_str();
        self.last_span = span;
        todo!();
    }

    fn consume_number(input: &'a str, mut span: Span) -> Span {
        let mut chars = input.char_indices();
        while let Some((b, c)) = chars.next() {
            match c {
                '0'..='9' => {
                    span.last_byte = Some(b);
                }
                _ => break,
            }
        }
        span
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token<'a> {
    Add,
    Sub,
    Mul,
    // Div,
    OpenParen,
    CloseParen,
    Number(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub line_num: usize,
    pub start: usize,
    first_byte: usize,
    last_byte: Option<usize>,
}

impl Span {
    pub fn union(self, other: Span) -> Self {
        Self {
            line_num: cmp::min(self.line_num, other.line_num),
            start: cmp::min(self.start, other.start),
            first_byte: cmp::min(self.first_byte, other.first_byte),
            last_byte: cmp::max(self.last_byte, other.last_byte),
        }
    }

    pub fn sub_str<'a>(&self, s: &'a str) -> Option<&'a str> {
        if self.first_byte >= s.as_bytes().len() {
            None
        } 
        else if let Some(last_byte) = self.last_byte {
            Some(&s[self.first_byte..last_byte])
        } else {
            Some(&s[self.first_byte..])
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line_num + 1, self.start + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_next() {
        let mut lexer = Lexer::new("(1 + 1)\n* 2");
        let expected = vec![
            Some((Token::OpenParen, Span {
                line_num: 0,
                start: 0,
                first_byte: 0,
                last_byte: Some(1),
            })),
            Some((Token::Number("1"), Span {
                line_num: 0,
                start: 1,
                first_byte: 1,
                last_byte: Some(2),
            })),
            Some((Token::Add, Span {
                line_num: 0,
                start: 4,
                first_byte: 4,
                last_byte: Some(5),
            })),
            Some((Token::Number("1"), Span {
                line_num: 0,
                start: 6,
                first_byte: 6,
                last_byte: Some(7),
            })),
            Some((Token::CloseParen, Span {
                line_num: 0,
                start: 7,
                first_byte: 7,
                last_byte: Some(8),
            })),
            Some((Token::Mul, Span {
                line_num: 1,
                start: 0,
                first_byte: 9,
                last_byte: Some(10),
            })),
            Some((Token::Number("2"), Span {
                line_num: 0,
                start: 0,
                first_byte: 12,
                last_byte: Some(13),
            })),
        ];
        for e in expected {
            assert_eq!(e, lexer.next());
        }
    }
}
