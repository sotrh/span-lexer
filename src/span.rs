use std::{fmt, ops::{Deref, Range, RangeInclusive}};

/// Represents the substring of a `&str`
#[derive(Debug, Clone, Copy)]
pub struct Span<'a> {
    data: &'a str,
    pub(crate) first_byte: usize,
    pub(crate) last_byte: usize,
}

impl<'a> Span<'a> {
    /// Creates a new span over all of `data`
    pub fn new(data: &'a str) -> Self {
        Self {
            data,
            first_byte: 0,
            last_byte: data.as_bytes().len(),
        }
    }

    /// Returns the substring of the `data`
    pub fn spanned(&self) -> &'a str {
        &self.data[self.first_byte..self.last_byte]
    }

    pub fn first_char(&self) -> Option<char> {
        self.spanned().chars().next()
    }

    /// Consume a range of characters
    /// Returns the consumed chars if any
    pub fn consume<CC: ContainsChar>(&mut self, valid: CC) -> Option<Self> {
        let mut chars = self.spanned().char_indices().peekable();
        let mut consumed_bytes = 0;
        // let mut consumed_chars = 0;
        while let Some((b, c)) = chars.peek() {
            if valid.contains_char(*c) {
                // In theory, we don't need `b` anymore
                consumed_bytes = b + bytes_per_char(*c);
                // consumed_chars += 1;
                chars.next();
            } else {
                break;
            }
        }

        if consumed_bytes > 0 {
            let old_first_byte = self.first_byte;
            let new_first_byte = self.first_byte + consumed_bytes;
            self.first_byte = new_first_byte;
            Some(Span {
                data: self.data,
                first_byte: old_first_byte,
                last_byte: new_first_byte,
            })
        } else {
            None
        }
    }
}

impl<'a> fmt::Display for Span<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}:{}", self.spanned(), self.first_byte, self.last_byte)
    }
}

impl Deref for Span<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.spanned()
    }
}

pub trait ContainsChar {
    fn contains_char(&self, c: char) -> bool;
}

impl ContainsChar for char {
    fn contains_char(&self, c: char) -> bool {
        *self == c
    }
}

impl<'a> ContainsChar for &'a str {
    fn contains_char(&self, c: char) -> bool {
        self.contains(c)
    }
}

impl ContainsChar for Range<char> {
    fn contains_char(&self, c: char) -> bool {
        self.contains(&c)
    }
}

impl ContainsChar for RangeInclusive<char> {
    fn contains_char(&self, c: char) -> bool {
        self.contains(&c)
    }
}

impl<A, B> ContainsChar for (A, B)
where A: ContainsChar, B: ContainsChar {
    fn contains_char(&self, c: char) -> bool {
        self.0.contains_char(c) || self.1.contains_char(c)
    }
}

fn bytes_per_char(c: char) -> usize {
    let u = c as usize;
    let mut bytes = 1;
    if u & 0x00_00_FF_00 != 0 {
        bytes += 1;
    }
    if u & 0x00_FF_00_00 != 0 {
        bytes += 2;
    }
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bytes_per_char() {
        assert_eq!(1, bytes_per_char('c'));
        assert_eq!(2, bytes_per_char('Č'));
        assert_eq!(4, bytes_per_char('🚀'));
    }

    #[test]
    fn first_char() {
        let mut span = Span::new("aaaa🚀🚀🚀aaaa");
        println!("{}", span.consume("a").unwrap());
        println!("{}", span.consume("🚀").unwrap());
        println!("{}", span.consume("a").unwrap());
    }

    #[test]
    fn spanned_new() {
        let expected = "as;ldfkj";
        let actual = Span::new(expected).spanned();
        assert_eq!(expected, actual);
    }

    #[test]
    fn spanned_consume() {
        let input = "     654324987321654";
        let mut span = Span::new(input);
        let consumed = span.consume(" ").unwrap();
        assert_eq!("     ", consumed.spanned());
        assert_eq!("654324987321654", span.spanned());
        let consumed = span.consume("0123456789").unwrap();
        assert_eq!("654324987321654", consumed.spanned());
        assert_eq!("", span.spanned());
    }

    #[test]
    fn spanned_empty() {
        let span = Span::new("");
        assert_eq!("", &*span);
    }
}