/*a Copyright

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

@file    lib.rs
@brief   Markup library
 */

//a Documentation
#![warn(missing_docs)]
// #![warn(missing_doc_code_examples)]
/*!

# Lexer library

This library provides a generic mechanism for parsing data into
streams of tokens.

This is commonly used in human-readable language compilers and
interpreters, to convert from a text stream into values that can then
be parsed according to the grammar of that language.§

A simple example would be for a calculator that operates on a stream
of numbers and mathematical symbols; the first step of processing that
the calculator must do is to convert the text stream into abstract
tokens such as 'the number 73' and 'the plus sign'. Once the
calculator has such tokens it can piece them together into a real
expression that it can then evaluate.

## Basic concept

The basic concept of a lexer is to convert a stream of (e.g.) [char]
into a stream of 'Token' - which will be specific to the lexer. The
lexer starts at the beginning of the text, and moves through consuming
characters into tokens.

## Lexer implementations

A lexer is not difficult to implement, and there are many alternative
approaches to doing so. A very simple approach for a [String] would be
to have a loop that matches the start of the string with possible
token values (perhaps using a regular expression), and on finding a
match it can 'trim' the front of the String, yield the token, and then
loop again.

This library provides an implementation option that gives the ability
to provide good error messages when things go wrong; it provides a
trait that allows abstraction of the lexer from the consumer (so that
one can get streams of tokens from a String, a BufRead, etc.); it
provides the infrastructure for any lexer using a simple mechanism for
parsing tokens.

# Positions in files

The crate provides some mechanisms for tracking the position of
parsing within a stream, so that error messages can be appropriately
crafted for the end user.

Tracking the position as a minimum is following the byte offset within
the file; additionally the line number and column number can also be
tracked.

As Rust utilizes UTF8 encoded strings, not all byte offsets correspond
to actual [char]s in a stream, and the column separation between two
characters is not the difference between their byte offsets. So traits
are provided to manage positions within streams, and to help with
reporting them.

The bare minimum though, does not require tracking of lines and
columns; only the byte offset tracking *has* to be used.

The [Lexer] is therefore generic on a stream position type: this must
be lightweight as it is moved around and copied frequently, and must
be static.

# Tokens

The token type that the [Lexer] produces from its parsing is supplied
by the client; this is normally a simple enumeration.

The parsing is managed by the [Lexer] with the client providing a
slice of matching functions; each matching function is applied in
turn, and the first that returns an Ok of a Some of a token yields the
token and advances the parsing state. The parsers can generate an
error if they detect a real error in the stream (not just a mismatch
to their token type).

# Error reporting

With the file position handling used within the [Lexer] it is possible
to display contextual error information - so if the whole text is
retained by the [Lexer] then an error can be displayed with the text
from the source with the error point/region highlighted.

Support for this is provided by the [FmtContext] trait, which is
implemented particularly for [LexerOfString].

!*/

//a Imports
mod char_stream;
mod lexer;
mod posn_and_span;

pub use char_stream::CharStream;
pub use char_stream::FmtContext;

pub use posn_and_span::LineColumn;
pub use posn_and_span::StreamCharPos;
pub use posn_and_span::StreamCharSpan;
pub use posn_and_span::{PosnInCharStream, UserPosn};

pub use crate::lexer::LexerOfStr;
pub use crate::lexer::LexerOfString;
pub use crate::lexer::ParserIterator;
pub use crate::lexer::SimpleParseError;
pub use crate::lexer::{BoxDynLexerParseFn, Lexer, LexerError, LexerParseFn, LexerParseResult};
