use crate::abstracts::{
	AbstractToken,
	AbstractSource,
	AbstractBoundary,
	ComparableAbstractSource,
	AbstractBoundaryCollection
};
use crate::helpers::count_tabs;
use crate::raw_token::RawToken;
use crate::token_info::TokenInfo;
use crate::special_characters::{EQUAL, POUND_SIGN};
use crate::{
	simplex,
	complex,
	attacher,
	line_comment,
	block_comment,
	line_othertongue,
	block_othertongue
};

/// Returns the info of first recognized token and its probably last seen index in the source.
///
/// It needs an array of bytes as the first argument (known as source), where to start looking for
/// the token as the second argument (known as the offset), the number of tabs to work in case it
/// found a block token of any kind (known as tab count), and a boolean if the line at the current
/// offset has already been checked (if this is true, it would check for scope level).
///
/// ## Notes
/// May panic if the last possible lexer has returned an unexpected token.
///
/// ## Examples
/// ```
/// use std::ops::Range;
/// use abstract_chearmyp_token::AbstractToken;
/// use chearmyp_lexer::any;
/// use chearmyp_token::Token;
///
/// let (token, last_index): (
///   Token<Range<usize>, Vec<Range<usize>>>,
///   usize
/// ) = any(&b"hello"[..], 0, 0, false);
/// assert_eq!(token, Token::new_complex(0..5));
/// assert_eq!(last_index, 5);
/// ```
pub fn any<T, U, V, W>(src: T, offset: usize, tab_count: usize, is_in_new_line: bool)
-> TokenInfo<W>
where
	T: AbstractSource + ComparableAbstractSource<&'static str> + Clone,
	U: AbstractBoundary<usize>,
	V: AbstractBoundaryCollection<usize, U>,
	W: AbstractToken<usize, U, usize, U, V> {
	let mut tabbed_offset = offset;

	if is_in_new_line {
		let new_tab_count = count_tabs(src.clone().forward_slice(offset), tab_count);
		if new_tab_count != tab_count {
			return (W::new_scope_level(new_tab_count), offset + new_tab_count);
		} else {
			tabbed_offset += tab_count;
		}
	}

	let mut offset = tabbed_offset;
	let mut raw_token;

	macro_rules! lex {
		(
			$parser:ident$(($($other_argument:tt),+))?
			$(unless $raw_token:ident($($content:tt),+) turns into $new_token:ident => $block:block)?
			$(
				which expects
					$expected_raw_token:ident($($expected_content:tt),+)
					turning into $expected_new_token:ident
			)?
		) => {
			let info = $parser(src.clone(), offset, $($($other_argument,)*)?);
			raw_token = info.0;
			offset = info.1;
			$(
				if let RawToken::$raw_token($($content,)+) = raw_token {
					let token = W::$new_token($($content,)+);
					(token, offset)
				} else $block
			)?
			$(
				if let RawToken::$expected_raw_token($($expected_content,)+) = raw_token {
					let token = W::$expected_new_token($($expected_content,)+);
					(token, offset)
				} else {
					let effect = "There is an unexpected raw token in lexing found in the source.";
					let cause = "This is possibly due to developer error.";
					panic!("{} {}", effect, cause);
				}
			)?
		};
	}

	if src.is_same_needle_at(offset, POUND_SIGN) {
		lex!{
			block_comment(tab_count)
			unless BlockComment(comment) turns into new_block_comment => {
				lex!{ line_comment which expects LineComment(comment) turning into new_line_comment }
			}
		}
	} else if src.is_same_needle_at(offset, EQUAL) {
		lex!{
			block_othertongue(tab_count)
			unless BlockOthertongue(othertongue) turns into new_block_othertongue => {
				lex!{
					line_othertongue
					unless LineOthertongue(othertongue) turns into new_line_othertongue => {
						lex!{
							attacher(offset)
							unless Attacher(label, content) turns into new_attacher => {
								let search_offset = if offset > tabbed_offset {
									offset - 1
								} else {
									tabbed_offset
								};
								let slice_start_offset = tabbed_offset;
								offset = slice_start_offset;

								lex!{
									simplex(search_offset)
									unless Simplex(concept) turns into new_simplex => {
										let search_offset = offset;
										let slice_start_offset = tabbed_offset;
										offset = slice_start_offset;
										lex!{
											complex(search_offset)
											which expects Complex(concept)
											turning into new_complex
										}
									}
								}
							}
						}
					}
				}
			}
		}
	} else {
		lex!{
			attacher(offset)
			unless Attacher(label, content) turns into new_attacher => {
				let search_offset = if offset > tabbed_offset {
					offset - 1
				} else {
					tabbed_offset
				};
				let slice_start_offset = tabbed_offset;
				offset = slice_start_offset;

				lex!{
					simplex(search_offset)
					unless Simplex(concept) turns into new_simplex => {
						let search_offset = offset;
						let slice_start_offset = tabbed_offset;
						offset = slice_start_offset;
						lex!{
							complex(search_offset)
							which expects Complex(concept)
							turning into new_complex
						}
					}
				}
			}
		}
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec};
	use crate::abstracts::AbstractToken;
	use crate::token::Token;

	use super::any;

	macro_rules! test_any {
		(
			source: $source:expr,
			expected token: $token_constructor:ident($($token_content:expr),+),
			expected last seen index: $last_seen_index:literal
		) => {
			test_any!{
				source: $source,
				offset: 0,
				expected token: $token_constructor($($token_content),+),
				expected last seen index: $last_seen_index
			}
		};
		(
			source: $source:expr,
			offset: $offset:literal,
			expected token: $token_constructor:ident($($token_content:expr),+),
			expected last seen index: $last_seen_index:literal
		) => {
			test_any!{
				source: $source,
				offset: $offset,
				tab count: 0,
				is in new line: false,
				expected token: $token_constructor($($token_content),+),
				expected last seen index: $last_seen_index
			}
		};
		(
			source: $source:expr,
			offset: $offset:literal,
			tab count: $tab_count:literal,
			is in new line: $is_in_new_line:literal,
			expected token: $token_constructor:ident($($token_content:expr),+),
			expected last seen index: $last_seen_index:literal
		) => {
			test_any!{
				source: $source,
				offset: $offset,
				tab count: $tab_count,
				is in new line: $is_in_new_line,
				info: (
					Token::<Range<usize>, Vec<Range<usize>>>::$token_constructor(
						$($token_content),+
					),
					$last_seen_index
				)
			}
		};
		(
			source: $source:expr,
			offset: $offset:literal,
			tab count: $tab_count:literal,
			is in new line: $is_in_new_line:literal,
			info: $expected_info:expr
		) => {
			let info = any::<
				&[u8],
				Range<usize>,
				Vec<Range<usize>>,
				Token<Range<usize>, Vec<Range<usize>>>
			>(
				&&$source[..],
				$offset,
				$tab_count,
				$is_in_new_line
			);
			assert_eq!(info, $expected_info);
		};
	}

	#[test]
	fn can_lex_line_comment() {
		test_any!(
			source: b"#abc",
			expected token: new_line_comment(1..4),
			expected last seen index: 4
		);
	}

	#[test]
	fn can_lex_empty_line_comment() {
		test_any!(
			source: b"#",
			expected token: new_line_comment(1..1),
			expected last seen index: 1
		);
	}

	#[test]
	fn can_lex_block_comment() {
		let mut expected_lines = Vec::new();
		expected_lines.push(4..7);

		test_any!(
			source: b"###\n\tde\n###",
			expected token: new_block_comment(expected_lines),
			expected last seen index: 11
		);
	}

	#[test]
	fn can_lex_simplex() {
		test_any!(
			source: b"efg|",
			expected token: new_simplex(0..3),
			expected last seen index: 4
		);
	}

	#[test]
	fn can_lex_complex() {
		test_any!(
			source: b"hi",
			expected token: new_complex(0..2),
			expected last seen index: 2
		);
	}

	#[test]
	fn can_lex_attacher() {
		test_any!(
			source: b"jklm:\tn",
			expected token: new_attacher(0..4, 6..7),
			expected last seen index: 7
		);
	}

	#[test]
	fn can_lex_line_othertongue() {
		test_any!(
			source: b"= o",
			expected token: new_line_othertongue(2..3),
			expected last seen index: 3
		);
	}

	#[test]
	fn can_lex_invalid_line_othertongue_into_complex() {
		test_any!(
			source: b"=o",
			expected token: new_complex(0..2),
			expected last seen index: 2
		);
	}

	#[test]
	fn can_lex_block_othertongue() {
		let mut expected_lines = Vec::new();
		expected_lines.push(4..8);

		test_any!(
			source: b"===\npqrs\n===",
			expected token: new_block_othertongue(expected_lines),
			expected last seen index: 12
		);
	}

	#[test]
	fn can_lex_indented_attacher_from_new_line() {
		test_any!(
			source: b"	tuvw:	x",
			offset: 0,
			tab count: 1,
			is in new line: true,
			expected token: new_attacher(1..5, 7..8),
			expected last seen index: 8
		);
	}
}
