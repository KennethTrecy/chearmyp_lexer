use crate::abstracts::{
	AbstractToken,
	AbstractBoundary,
	AbstractSource,
	AbstractTokenQueue,
	AbstractScopeLevelToken,
	AbstractBoundaryCollection,
	ComparableAbstractSource
};
use crate::token::TokenKind;
use crate::any;
use crate::special_characters::NEW_LINE;

/// Returns a stream of tokens based from the source.
///
/// The source is the first argument which contain an array of bytes. This is the main lexer.
///
/// ## Examples
/// ```
/// use std::ops::Range;
/// use std::collections::VecDeque;
/// use abstract_chearmyp_token::{AbstractToken, AbstractTokenQueue};
/// use chearmyp_lexer::lex;
/// use chearmyp_token::Token;
/// let source = b"
/// a complex
/// 	this:	is an attacher
/// 	a simplex|
/// ## This is a line comment
/// ";
///
/// let queue: VecDeque<
/// 	Token<
/// 		Range<usize>,
/// 		Vec<Range<usize>>
/// 	>
/// > = lex(&&source[..], VecDeque::new());
///
/// assert_eq!(queue[0], Token::<Range<usize>, Vec<Range<usize>>>::new_complex(1..10));
/// assert_eq!(queue[1], Token::<Range<usize>, Vec<Range<usize>>>::new_scope_level(1));
/// assert_eq!(queue[2], Token::<Range<usize>, Vec<Range<usize>>>::new_attacher(12..16, 18..32));
/// assert_eq!(queue[3], Token::<Range<usize>, Vec<Range<usize>>>::new_simplex(34..43));
/// assert_eq!(queue[4], Token::<Range<usize>, Vec<Range<usize>>>::new_scope_level(0));
/// assert_eq!(queue[5], Token::<Range<usize>, Vec<Range<usize>>>::new_line_comment(46..69));
/// ```
pub fn lex<T, U, V, W, X, Y>(src: &T, mut token_queue: Y) -> Y
where
	T: AbstractSource + ComparableAbstractSource<&'static str> + Clone,
	U: AbstractBoundary<usize>,
	V: AbstractBoundaryCollection<usize, U>,
	W: AbstractToken<usize, U, usize, U, V> + From<X>,
	X: AbstractScopeLevelToken + From<W>,
	Y: AbstractTokenQueue<usize, U, usize, U, V, W> {
	let mut tab_count = 0;
	let mut scanned_size = 0;
	let mut is_in_new_line = false;

	while !src.is_empty_at(scanned_size) {
		if src.is_same_needle_at(scanned_size, NEW_LINE) {
			scanned_size += 1;
			is_in_new_line = true;
			continue;
		}

		let (token, last_seen_index) = any(src.clone(), scanned_size, tab_count, is_in_new_line);
		if W::kind(&token) == TokenKind::ScopeLevel {
			let scope_level = X::from(token);
			let new_scope_level = X::level(&scope_level);
			tab_count = new_scope_level;
			let token = W::from(scope_level);
			token_queue.push_token(token);
		} else {
			token_queue.push_token(token);
		}

		scanned_size = last_seen_index;
		is_in_new_line = false;
	}

	token_queue
}


#[cfg(test)]
mod t {
	use crate::native::{Range, Vec, VecDeque};
	use crate::abstracts::{SimpleAbstractToken, AbstractTokenQueue};
	use crate::token::Token;

	use super::lex;

	#[test]
	fn can_lex_line_comment() {
		let source = b"# Hello World";
		let mut expected_token_queue = VecDeque::new();
		expected_token_queue.push_token(Token::<Range<usize>, Vec<Range<usize>>>::new_line_comment(1..13));

		let token_queue: VecDeque<
			Token<Range<usize>, Vec<Range<usize>>>
		> = lex(&&source[..], VecDeque::new());

		assert_eq!(token_queue, expected_token_queue);
	}

	#[test]
	fn can_lex_block_comment() {
		let source = b"###\nHello world\n###";
		let first_index = 4;
		let last_index = source.len() - 4;
		let expected_lines = vec![first_index..last_index];
		let mut expected_token_queue = VecDeque::new();
		expected_token_queue.push_token(Token::<Range<usize>, Vec<Range<usize>>>::new_block_comment(expected_lines));

		let token_queue: VecDeque<
			Token<Range<usize>, Vec<Range<usize>>>
		> = lex(&&source[..], VecDeque::new());

		assert_eq!(token_queue, expected_token_queue);
	}

	#[test]
	fn can_lex_simplex() {
		let source = b"hello_world|";
		let last_index = source.len() - 1;
		let mut expected_token_queue = VecDeque::new();
		expected_token_queue.push_token(Token::<Range<usize>, Vec<Range<usize>>>::new_simplex(0..last_index));

		let token_queue: VecDeque<
			Token<Range<usize>, Vec<Range<usize>>>
		> = lex(&&source[..], VecDeque::new());

		assert_eq!(token_queue, expected_token_queue);
	}

	#[test]
	fn can_lex_complex() {
		let source = b"HelloWorld";
		let mut expected_token_queue = VecDeque::new();
		expected_token_queue.push_token(Token::<Range<usize>, Vec<Range<usize>>>::new_complex(0..source.len()));

		let token_queue: VecDeque<
			Token<Range<usize>, Vec<Range<usize>>>
		> = lex(&&source[..], VecDeque::new());

		assert_eq!(token_queue, expected_token_queue);
	}

	#[test]
	fn can_lex_attacher() {
		let source = b"hello:	world";
		let mut expected_token_queue = VecDeque::new();
		expected_token_queue.push_token(Token::<Range<usize>, Vec<Range<usize>>>::new_attacher(0..5, 7..12));

		let token_queue: VecDeque<
			Token<Range<usize>, Vec<Range<usize>>>
		> = lex(&&source[..], VecDeque::new());

		assert_eq!(token_queue, expected_token_queue);
	}

	#[test]
	fn can_lex_line_othertongue() {
		let source = b"= hello-world";
		let first_index = 2;
		let mut expected_token_queue = VecDeque::new();
		expected_token_queue.push_token(Token::<Range<usize>, Vec<Range<usize>>>::new_line_othertongue(first_index..source.len()));

		let token_queue: VecDeque<
			Token<Range<usize>, Vec<Range<usize>>>
		> = lex(&&source[..], VecDeque::new());

		assert_eq!(token_queue, expected_token_queue);
	}

	#[test]
	fn can_lex_block_othertongue() {
		let source = b"===\nhelloWorld\n===";
		let first_index = 4;
		let last_index = source.len() - 4;
		let expected_lines = vec![first_index..last_index];
		let mut expected_token_queue = VecDeque::new();
		expected_token_queue.push_token(Token::<Range<usize>, Vec<Range<usize>>>::new_block_othertongue(expected_lines));

		let token_queue: VecDeque<
			Token<Range<usize>, Vec<Range<usize>>>
		> = lex(&&source[..], VecDeque::new());

		assert_eq!(token_queue, expected_token_queue);
	}

	#[test]
	fn can_lex_empty_scopes() {
		let source = b"\t\t";
		let mut expected_token_queue = VecDeque::new();
		expected_token_queue.push_token(
			Token::<Range<usize>, Vec<Range<usize>>>::new_scope_level(2)
		);

		let token_queue: VecDeque<
			Token<Range<usize>, Vec<Range<usize>>>
		> = lex(&&source[..], VecDeque::new());

		assert_eq!(token_queue, expected_token_queue);
	}
}
