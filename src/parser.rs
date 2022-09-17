//a Imports
use crate::lexer::{TextPos, TokenTypeError};

//a Token, Stream, PFnError, PResult
//tt Parser
/// A parser defined using assocaited types
///
/// It is possible to change the type input to be Sized + Copy, and to
/// pull the 'get_token' function into the Parser trait itself rather
/// than applying it to the Input type.
///
/// However, if that is done then there is nothing tying the Input to
/// the Parser type, only the Parser to the Input; the get_token
/// invocation becomes a P::get_token(input), which is fine, but any
/// use of a parser function (which requires P:Parser and I:Input to
/// be specified) cannot derive the P type from the I type (which is
/// well known as the input to the get_token function). Hence going
/// down that path requires a lot more turbofish on parser_fn
/// invocations to specifiy the Parser itself, which is an
/// anti-pattern.
pub trait Parser: Sized {
    /// The type of tokens that are parsed by the Parser; a stream of
    /// these is effectively the input stream to the Parser
    type Token;

    /// An error type that the parser returns if there is a failure to
    /// parse a input stream of tokens
    ///
    /// Using the lexer the Error type will often be:
    ///
    ///    Error : lexer::TokenTypeError<Pos : lexer::TextPos>
    /// 
    /// Pos is the type of a position in an input that needs to be
    /// reported for errors, or that is traced within the Tokens;
    /// often this is the file and start/end lines/characters of the
    /// token
    ///
    /// 
    type Error; // : TokenTypeError<Self::Pos>;

    /// The input type that provides the 'get_token' function to
    /// attempt to provide the next token for a parser to try to
    /// consume.  This type must support Clone as cheaply as possible,
    /// ideally it should be Copy. This is because the parser must
    /// keep copies of the input state so that it can backtrack on
    /// parsing failures.
    type Input: ParserFnInput<Self>;
}

//tp ParserInputResult
/// This is the result of the Parser::Input::get_token function,
/// which takes 
///
/// P:Parser
pub type ParserInputResult<P> =
    Result<Option<(<P as Parser>::Input, <P as Parser>::Token)>, <P as Parser>::Error>;

//tt ParserFnInput
/// Trait required by a parser of its input
///
/// The parser invokes this to get the tokens that it needs to match;
/// making it belong to the Input allows a get_token() call to infer
/// the type of the Parser that it is associated with, reducing
/// turbofish annotation requirements.
///
/// Requiring Copy here allows parser functions to manipulate the
/// input simply without explicit cloning
pub trait ParserFnInput<P: Parser<Input = Self>>: Copy {
    fn get_token(&self) -> ParserInputResult<P>;
}

//tp ParserResult
#[derive(Debug)]
pub enum ParserResult<P: Parser, R> {
    Mismatched,
    Matched(P::Input, R),
}

//tp ParserFnResult
// P:Parser<'a, Error = E>
pub type ParserFnResult<P, R> = Result<ParserResult<P, R>, <P as Parser>::Error>;

/*
struct ParserFnResultv2<'a, P: Parser<'a>, R>(ParserFnResult<'a, P, R>);

impl<'a, P: Parser<'a>, R> std::ops::Deref for ParserFnResultv2<'a, P, R> {
    type Target = ParserFnResult<'a, P, R>;
    fn deref(&self) -> &ParserFnResult<'a, P, R> {
        &self.0
    }
}
*/

/* Try
impl <'a, P: Parser<'a>, R> std::ops::Try for ParserFnResultv2 <'a, P, R> {
type Output = ParserResult<'a, P, R>;
type Residual = <P as Parser<'a>>::Error;
    fn from_output(x: Self::Output) -> Self {
        Self(Ok(x))
    }
    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        self.0.branch()
    }
}
impl <'a, P: Parser<'a>, R> std::ops::FromResidual<R> for ParserFnResultv2 <'a, P, R> {
    fn from_residual(residual: <P as Parser>::Error) -> Self {
        Self(Err(residual))
    }
}

impl<'a, P: Parser<'a>, R>  ParserFnResultv2 <'a, P, R> {
    pub fn mismatch() -> Self {
       Self( Ok(ParserResult::Mismatched))
    }
    pub fn matched(input:P::Input, r:R) -> Self {
        Self(Ok(ParserResult::Matched(input, r)))
    }
    pub fn or<F>(self, stream:P::Input, f:&F) -> Self
    where F: Fn(P::Input) -> ParserFnResultv2<'a, P, R> {
        match self.0 {
            Ok(ParserResult::Mismatched) => f(stream),
            _ => self,
        }
    }
}

fn map_token<'a, P, I: ParserFnInput<'a, P>, R, F>(
    f: F,
) -> impl Fn(I) -> ParserFnResultv2<'a, P, R>
where
    P: Parser<'a, Input = I>,
    F: Fn(P::Token) -> Option<R>,
{
    move |input| {
        match input.get_token()? {
            Some((input, token)) => {
                if let Some(r) = f(token) {
                    return ParserFnResultv2::matched(input, r);
                }
            }
            _ => (),
        }
        ParserFnResultv2::mismatch()
    }
}
*/
