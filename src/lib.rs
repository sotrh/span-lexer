mod span;
pub use span::*;

/// A lexer powered by 
pub struct Lexer<'a> {
    span: Span<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            span: Span::new(data),
        }
    }

    pub fn next(&mut self) -> Option<Token> {
        let mut token = None;
        while let Some(c) = self.span.first_char() {
            match c {
                ' ' | '\t' | '\n' => {
                    self.span.consume(" \t\n").unwrap();
                }
                '_' | 'A'..='z' => {
                    token = self.span.consume(('_', 'A'..='z')).map(|s| Token::Word(s));
                    break;
                }
                '.' | ',' | '!' | '?' | ';' | ':' => {
                    token = self.span.consume(".,!?;:").map(|s| Token::Punct(s));
                    break;
                }
                _ => unimplemented!("Unsupported char '{}'", c),
            }
        }
        token
    }
}

#[derive(Debug)]
pub enum Token<'a> {
    Word(Span<'a>),
    Punct(Span<'a>),
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn tokens_match(t0: Option<Token>, t1: Option<Token>) -> bool {
        match t0 {
            Some(t0) => {
                match t1 {
                    Some(t1) => {
                        match (t0, t1) {
                            (Token::Word(s0), Token::Word(s1)) => {
                                spans_match(s0, s1)
                            }
                            (Token::Punct(s0), Token::Punct(s1)) => {
                                spans_match(s0, s1)
                            }
                            _ => false
                        }
                    }
                    None => false
                }
            }
            None => t1.is_none()
        }
    }

    fn spans_match(s0: Span<'_>, s1: Span<'_>) -> bool {
        // Ignore the byte offsets for now
        let s0 = s0.spanned();
        let s1 = s1.spanned();
        s0 == s1
    }

    fn span(data: &str) -> Span<'_> {
        Span::new(data)
    }

    #[test]
    fn lexer_next() {
        let input = "This is\n a test!";
        let mut lexer = Lexer::new(input);
        let expected = vec![
            Some(Token::Word(span("This"))),
            Some(Token::Word(span("is"))),
            Some(Token::Word(span("a"))),
            Some(Token::Word(span("test"))),
            Some(Token::Punct(span("!"))),
        ];
        for e in expected {
            let t = lexer.next();
            println!("{:?}, {:?}", e, t);
            assert!(tokens_match(e, t));
        }
    }

    #[test]
    fn readme() {
        let mut lexer = Lexer::new("This is a test. 🚀");
        let token = lexer.next();
        match token {
            Some(Token::Word(span)) => {
                assert_eq!("This", &*span);
            }
            _ => unreachable!(),
        }
    }
}