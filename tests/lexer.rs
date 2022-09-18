use lexer::{SimpleToken, TokenParseError, TSSLexer};
use lexer::{LexerParseResult};

type P = usize;
type K = u64;
type Token = SimpleToken<P, K>;
type E = TokenParseError<P>;
type L<'a> = TSSLexer<'a, P, Token, E>;

fn parse_keywords<'a>(l: &L<'a>, state:P, c: char) -> LexerParseResult<L<'a>>
where 'a: 'a
{
    let keywords: [(&[u8], u64); 2] = [(b"let", 0), (b"if", 1)];
    Token::parse_keyword(l, state, c, &keywords)
}

fn parse_whitespace<'a>(l:&L<'a>, state: P, c:char) -> LexerParseResult<L<'a>>
where 'a: 'a
{
    Token::parse_whitespace(l, state, c)
}

fn parse_id<'a>(l:&L<'a>, state: P, c:char) -> LexerParseResult<L<'a>>
where 'a: 'a
{
    Token::parse_id(l, state, c, char::is_alphabetic, char::is_alphanumeric)
}

//a Tests
#[test]
fn test_me() {
    let a = r##"let add x y = x + y; add 2 3
"##;

    let span = L::new(a);

    /*
    // Note must use closures here as Rust cannot resolve the lifetimes of the functions otherwise
    let parsers = [
        |l, state, c| Token::parse_digits(l, state, c),
        parse_whitespace,
        parse_keywords,
        parse_id,
        |l, state, c| Token::parse_char(l, state, c),
    ];
*/


    let parsers = [];
    for t in span.iter_tokens(&parsers) {
        println!("{:?}", t.unwrap());
        panic!("Done");
    }
    // assert!(false);
}
