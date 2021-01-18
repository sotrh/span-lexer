# Span Lexer

A lexer that returns Tokens with the span information of the original text. The lexer does not modify the text it lexes, nore does it allocate. **Note: this is just an example. This project is a proof of concept and not designed to be used.** That being said, the code is MIT/APACHE at the users discretion.

The implementation here leverages the `CharIndices` trait on `&str` to compute the start and end of a span. This example has been built to support most UTF-8 characters including multi byte characters such as 🚀. The core of the code is the `Span` struct found in `src/span.rs`.

## Usage

```rust
let mut lexer = Lexer::new("This is a test. 🚀");
let token = lexer.next();
match token {
    Some(Token::Word(span)) => {
        assert_eq!("This", &*span);
    }
    _ => unreachable!(),
}
```

Span implements `Deref` so you can use it as a `&str`. You can also use `span.spanned()` to achieve the same effect.

## Missing Features / Flaws

The biggest flaw by far is the `bytes_per_char()` function:

```rust
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
```

I'm not an expert in UTF-8, and I was wanting to get a working prototype. There are likely some examples where this code returns the wrong value, so feel free to submit a pull request!

Also, currently the lexer only produces tokens for the very simple `Token` struct I implemented for testing. A future version could support a more robust token set, or allow the user to supply there own custom token type.

Finally (AFAIK) storing the spans directly in the `Token` enum can make constructing tests more difficult. The lexer could be modified to return a tuple of `(Token, Span)`, but I didn't feel it necessary for this example.