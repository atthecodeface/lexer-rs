//a Imports
use crate::{ParserInput, ParserInputStream, ParseFnResult, ParseResult};

//a Pair / tuples
//fp pair
many_f_many_r! {
    /// Generate a parser function that combines a pair of parsers
    /// that must both match.
    ///
    /// The result of the parser is a pair of the results of the individual parsers
    pair, ( f1: F1 : R1, f2 : F2 : R2, ), (R1, R2), stream {
        use ParseResult::*;
        let (stream, r1) = {
            match f1(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        let (stream, r2) = {
            match f2(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        Ok(Matched(stream, (r1, r2)))
    }
}

//fp tuple3
many_f_many_r! {
    /// Generate a parser function that combines three parsers
    /// that must all match.
    ///
    /// The result of the parser is a triplet of the results of the
    /// individual parsers.
    tuple3, ( f1: F1 : R1, f2 : F2 : R2, f3 : F3 : R3), (R1, R2, R3), stream {
        use ParseResult::*;
        let (stream, r1) = {
            match f1(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        let (stream, r2) = {
            match f2(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        let (stream, r3) = {
            match f3(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        Ok(Matched(stream, (r1, r2, r3)))
    }
}

//fp tuple4
many_f_many_r! {
    /// Generate a parser function that combines four parsers
    /// that must all match.
    ///
    /// The result of the parser is a 4-tuple of the results of the
    /// individual parsers.
    tuple4, ( f1: F1 : R1, f2 : F2 : R2, f3 : F3 : R3, f4 : F4 : R4), (R1, R2, R3, R4), stream {
        use ParseResult::*;
        let (stream, r1) = {
            match f1(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        let (stream, r2) = {
            match f2(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        let (stream, r3) = {
            match f3(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        let (stream, r4) = {
            match f4(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        Ok(Matched(stream, (r1, r2, r3, r4)))
    }
}

//a delimited, preceded, succeeded, separated_pair
//fp separated_pair
many_f_many_r! {
    /// Generate a parser function that combines three parsers 'a',
    /// 'sep', 'b' and which returns a pair of the parsed matched
    /// content for 'a' and 'b' if all three match.
    ///
    /// An example use would be to parse <name> ';' <type>
    separated_pair, ( f1: F1 : R1, f2 : F2 : R2, f3 : F3 : R3), (R1, R3), stream {
        use ParseResult::*;
        let (stream, r1) = {
            match f1(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        let (stream, _r2) = {
            match f2(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        let (stream, r3) = {
            match f3(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        Ok(Matched(stream, (r1, r3)))
    }
}

//fp delimited
many_f_many_r! {
    /// Generate a parser function that combines three parsers 'pre',
    /// 'content', 'post' and which returns the parsed matched content
    /// if all three match.
    ///
    /// An example use would be to parse '(' <expr>> ')'
    delimited, ( f1: F1 : R1, f2 : F2 : R2, f3 : F3 : R3), R2, stream {
        use ParseResult::*;
        let (stream, _r1) = {
            match f1(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        let (stream, r2) = {
            match f2(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        let (stream, _r3) = {
            match f3(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        Ok(Matched(stream, r2))
    }
}

//fp preceded
many_f_many_r! {
    /// Generate a parser function that combines two parsers 'pre',
    /// and 'content' and which returns the parsed matched content
    /// if both match.
    ///
    /// An example use would be to parse 'clock' <signal_id>
    preceded, ( f1: F1 : R1, f2 : F2 : R2), R2, stream {
        use ParseResult::*;
        let (stream, _r1) = {
            match f1(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        let (stream, r2) = {
            match f2(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        Ok(Matched(stream, r2))
    }
}

//fp succeeded
many_f_many_r! {
    /// Generate a parser function that combines two parsers 'content',
    /// and 'post' and which returns the parsed matched content
    /// if both match.
    ///
    /// An example use would be to parse <statement> ';'
    succeeded, ( f1: F1 : R1, f2 : F2 : R2), R1, stream {
        use ParseResult::*;
        let (stream, r1) = {
            match f1(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        let (stream, _r2) = {
            match f2(stream)? {
                Matched(a, b) => (a, b),
                _ => {
                    return Ok(Mismatched);
                }
            }
        };
        Ok(Matched(stream, r1))
    }
}
