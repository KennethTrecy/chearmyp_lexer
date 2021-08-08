#![no_std]
extern crate alloc;

/// Contains macros useful in tests
#[cfg(test)]
#[macro_use]
mod test_macros;

/// Contains the type alias used and/or returned by some lexers.
mod token_info;

/// Contains the data structures and type aliases used and/or returned by most lexers. They can be
/// used by lexers only.
mod raw_token;

/// Contains different characters needed to be recognized by the different lexers.
pub mod special_characters;

/// Contains `find_line_ending()`.
mod find_line_ending;

/// Contains `line_comment()` lexer.
mod line_comment;

/// Contains `block_comment()` lexer.
mod block_comment;

/// Contains `simplex()` lexer.
mod simplex;

/// Contains `complex()` lexer and `determine_ending()`.
mod complex;

/// Contains `attacher()` lexer.
mod attacher;

/// Contains `line_othertongue()` lexer.
mod line_othertongue;

/// Contains `block()` lexer.
mod block;

/// Contains `block_othertongue()` lexer.
mod block_othertongue;

/// Contains types of delimeter that lexers search for.
mod delimeter;

/// Contains `any()` lexer.
mod any;

/// Contains `count_tabs()` counter.
mod count_tabs;

/// Contains the general lexer.
mod lex;

pub use block::block;
pub use find_line_ending::find_line_ending;
pub use line_comment::line_comment;
pub use block_comment::block_comment;
pub use simplex::simplex;
pub use complex::complex;
pub use attacher::attacher;
pub use line_othertongue::line_othertongue;
pub use block_othertongue::block_othertongue;
pub use any::any;
pub use raw_token::RawToken;
pub use token_info::TokenInfo;
pub use lex::lex;
