//a Documentation
//a Imports
use std::ops::Range;

#[derive(Debug)]
struct TokenStream<'a> {
    text : &'a str,
}

#[derive(Debug, Copy, Clone)]
struct Pos {
    byte_ofs : usize,
    line : usize,
    column : usize,
}

impl std::default::Default for Pos {
    fn default() -> Self {
        Self { byte_ofs:0, line:1, column:1 }
    }
}

impl Pos {
    fn advance_cols(mut self, byte_ofs:usize, num_chars:usize) -> Self {
        self.byte_ofs = byte_ofs;
        self.column += num_chars;
        self
    }
    fn advance_line(mut self, byte_ofs:usize) -> Self {
        self.column = 1;
        self.line += 1;
        self.byte_ofs = byte_ofs;
        self
    }
}

#[derive(Debug, Copy, Clone)]
struct Span {
    start : Pos,
    end : Pos,
}

impl Span {
    fn new(start:Pos, end:Pos) -> Self {
        Self { start, end }
    }
    fn byte_range(&self) -> Range<usize> {
        let start = self.start.byte_ofs;
        let end = self.end.byte_ofs;
        Range { start, end }
    }
}

#[derive(Debug, Copy, Clone)]
struct TokenStreamSpan<'a> {
    stream : &'a TokenStream<'a>,
    end : usize,
    pos : Pos,
}

#[derive(Debug, Copy, Clone)]
enum Token {
    Newline(Pos),
    Char(Pos, char),
    OpenBra(Pos, char),
    CloseBra(Pos, char),
    Whitespace(Span),
    Documentation(Span),
    Id(Span),
    Digits(Span),
}

#[derive(Debug, Clone)]
struct Thing<'a> {
    stream : &'a TokenStream<'a>,
    pos : Pos,
    enumerator : std::iter::Enumerate<std::str::CharIndices<'a>>,
}

impl <'a> Thing<'a> {
    fn new(stream :&'a TokenStream<'a>) -> Self {
        let enumerator = stream.text.char_indices().enumerate();
        let pos = Pos::default();
        Self { stream, pos, enumerator }
    }
    fn get_token(&mut self) -> Option<Token> {
        if let Some((char_num, (byte_ofs, ch))) = self.enumerator.next() {
            let pos = self.pos;
            let byte_len = ch.len_utf8();
            let next_byte_ofs = byte_ofs + byte_len;
            if ch.is_ascii_digit() {
                let mut fwd = self.enumerator.clone();
                loop {
                    if let Some((char_num, (byte_ofs, ch))) = fwd.next() {
                        if !ch.is_ascii_digit() {
                            break;
                        }
                        self.enumerator.next();
                    } else {
                        break;
                    }
                }
                let n = byte_ofs - pos.byte_ofs;
                self.pos.advance_cols(n, n);
                Some(Token::Digits(Span::new(pos, self.pos)))
            } else {
                match ch {
                    '\n' => {
                        self.pos.advance_line(next_byte_ofs);
                        Some(Token::Newline(pos))
                    }
                    '(' | '[' | '{' => {
                        self.pos.advance_cols(next_byte_ofs, 1);
                        Some(Token::OpenBra(pos, ch))
                    }
                    ')' | ']' | '}' => {
                        self.pos.advance_cols(next_byte_ofs, 1);
                        Some(Token::OpenBra(pos, ch))
                    }
                    ' ' | '\t' => {
                        self.pos.advance_cols(next_byte_ofs, 1);
                        Some(Token::Whitespace(Span::new(pos, self.pos)))
                    }
                    ch => {
                        self.pos.advance_cols(next_byte_ofs, 1);
                        Some(Token::Char(pos, ch))
                    }
                }
            }
        } else {
            None
        }
    }
}

impl <'a> TokenStreamSpan<'a> {
    fn new(stream :&'a TokenStream<'a>) -> Self {
        Self { stream, end:stream.text.len(), pos:Pos::default() }
    }
    fn consumed_text(mut self, num_bytes:usize, num_chars: usize) -> Self {
        self.pos.advance_cols(num_bytes, num_chars);
        self
    }
    fn text<'b> (&'b self) -> &'b str {
        self.stream.get_span(self.pos.byte_ofs, self.end)
    }
}

impl <'a> TokenStream<'a> {
    pub fn new(text:&'a str) -> Self {
        Self { text }
    }
    pub fn as_span<'b>(&'b self) -> TokenStreamSpan<'b> {
        TokenStreamSpan::new(self)
    }
    fn get_span<'b> (&'b self, start:usize, end:usize) -> &'b str {
        unsafe { self.text.get_unchecked(start .. end) }
    }
    fn peek(&self, byte_ofs:usize) -> Option<char> {
        if byte_ofs >= self.text.len() {
            None
        } else {
            let text = unsafe {
                self.text.get_unchecked(byte_ofs .. self.text.len())
            };
            text.chars().nth(0)
        }
    }
}

#[test]
fn test_me() {
    let a = r##"let add x y = x + y; add 2 3
"##;
    let stream = TokenStream::new(a);
    let span = stream.as_span();
    let mut thing = Thing::new(&stream);
    while let Some(t) = thing.get_token() {
        println!("{:?}",t);
    }
    assert!(false);
}

    
