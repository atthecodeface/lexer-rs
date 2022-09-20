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
#![warn(missing_doc_code_examples)]
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
parsing tokens; and it provides a [SimpleToken] and parsing functions
enabling ver low overhead lexers for simple tasks.

# Positions in files




  !*/

//a Imports
mod lexer;
mod posn_and_span;
// mod text_stream;
// pub use simple::{SimpleKeyword, SimpleToken};

pub use posn_and_span::LineColumn;
pub use posn_and_span::StreamCharPos;
pub use posn_and_span::StreamCharSpan;
pub use posn_and_span::{PosnInCharStream, PosnInStream};
pub use posn_and_span::FmtContext;

pub use crate::lexer::LexerOfStr;
pub use crate::lexer::LexerOfString;
pub use crate::lexer::SimpleParseError;
pub use crate::lexer::ParserIterator;
pub use crate::lexer::{
    BoxDynLexerParseFn, Lexer, LexerError, LexerOfChar, LexerParseFn, LexerParseResult,
};
