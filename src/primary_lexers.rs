/// Contains `line_comment()` lexer.
mod line_comment;

/// Contains `block_comment()` lexer.
mod block_comment;

/// Contains `simplex()` lexer.
mod simplex;

/// Contains `complex()` lexer and `determine_ending()`.
mod complex;

// /// Contains `attacher()` lexer.
mod attacher;

/// Contains `line_othertongue()` lexer.
mod line_othertongue;

/// Contains `block_othertongue()` lexer.
mod block_othertongue;

pub use complex::complex;
pub use simplex::simplex;
pub use attacher::attacher;
pub use line_comment::line_comment;
pub use block_comment::block_comment;
pub use line_othertongue::line_othertongue;
pub use block_othertongue::block_othertongue;
