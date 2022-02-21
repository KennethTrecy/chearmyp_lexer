#![cfg_attr(feature = "no_std", no_std)]

//! # Chearmyp Lexer
//! Please read the README.md for more information.
//!
//! ## Features available
//! - `no_std`: Uses the `core` crate instead of `std` crate.

#[cfg(feature = "no_std")]
#[macro_use]
extern crate alloc;

#[cfg(test)]
mod native {
	#[cfg(feature = "no_std")]
	pub use core::ops::Range;

	#[cfg(feature = "no_std")]
	pub use alloc::{
		vec::Vec,
		collections::VecDeque
	};

	#[cfg(not(feature = "no_std"))]
	pub use std::{
		vec::Vec,
		ops::Range,
		collections::VecDeque
	};
}

mod abstracts {
	pub use abstract_chearmyp_source::{
		AbstractSource,
		AbstractSourceCollection,
		ComparableAbstractSource
	};

	pub use abstract_chearmyp_boundary::{
		AbstractBoundary,
		AbstractBoundaryCollection
	};

	pub use abstract_chearmyp_token::{
		AbstractToken,
		AbstractTokenQueue,
		AbstractScopeLevelToken
	};

	#[cfg(test)]
	pub use abstract_chearmyp_token::{
		SimpleAbstractToken
	};
}

mod token {
	#[cfg(test)]
	pub use chearmyp_token::Token;
	pub use abstract_chearmyp_token::TokenKind;
}

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

/// Contains types of delimeter that lexers search for.
mod delimeter;

/// Contains helper functions
pub mod helpers;

/// Contains the lexers which create token usable for lexing and parsing.
pub mod secondary_lexers;

/// Contains the lexers which create token usable for lexing only.
pub mod primary_lexers;

pub use raw_token::RawToken;
pub use token_info::TokenInfo;
pub use secondary_lexers::{lex, any};

use primary_lexers::{
	complex,
	simplex,
	attacher,
	line_comment,
	block_comment,
	line_othertongue,
	block_othertongue
};
