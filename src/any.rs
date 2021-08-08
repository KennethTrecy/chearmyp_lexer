use alloc::vec::Vec;
use crate::abstracts::AbstractToken;
use crate::TokenInfo;
use crate::raw_token::RawToken;
use crate::special_characters::{EQUAL, POUND_SIGN, SPACE};
use crate::{
	simplex,
	complex,
	attacher,
	line_comment,
	block_comment,
	line_othertongue,
	block_othertongue};

/// Returns the info of first recognized token and its probably last seen index in the source.
///
/// It needs an array of bytes as the first argument (known as source), where to start looking for
/// the token as the second argument (known as the offset), and the number of tabs to work in case
/// it found a block token of any kind.
///
/// ## Notes
/// May panic if the last possible lexer has returned an unexpected token.
///
/// ## Examples
/// ```
/// use chearmyp_lexer::any;
/// use chearmyp_lexer::Token;
///
/// let (any, last_index) = any(&b"hello"[..], 0, 0);
/// if let Token::Complex(content) = any {
/// 	assert_eq!(content, &b"hello"[..]);
/// } else {
/// 	panic!("The returned token is not complex.");
/// }
/// assert_eq!(last_index, 5);
/// ```
pub fn any<'a, T>(src: &'a [u8], offset: usize, tab_count: usize) -> TokenInfo<T>
where
	T: AbstractToken<Source = &'a [u8], SourceCollection = Vec<&'a [u8]>> {
	let original_offset = offset;
	let mut raw_token;
	let mut offset = offset;

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
			let info = $parser(src, offset, $($($other_argument,)*)?);
			raw_token = info.0;
			offset = info.1;
			$(
				if let RawToken::$raw_token($($content,)+) = raw_token {
					let token = AbstractToken::$new_token($($content,)+);
					(token, offset)
				} else $block
			)?
			$(
				if let RawToken::$expected_raw_token($($expected_content,)+) = raw_token {
					let token = AbstractToken::$expected_new_token($($expected_content,)+);
					(token, offset)
				} else {
					panic!("There is an unxpected raw token in lexing the source.");
				}
			)?
		};
	}

	if src[0] == POUND_SIGN {
		lex!{
			block_comment(tab_count)
			unless BlockComment(comment) turns into new_block_comment => {
				lex!{ line_comment which expects LineComment(comment) turning into new_line_comment }
			}
		}
	} else if src[0] == EQUAL || (src[0] == SPACE && src[1] == EQUAL) {
		lex!{
			block_othertongue(tab_count)
			unless BlockOthertongue(othertongue) turns into new_block_othertongue => {
				lex!{
					line_othertongue
					which expects LineOthertongue(othertongue)
					turning into new_line_othertongue
				}
			}
		}
	} else {
		lex!{
			attacher(0)
			unless Attacher(label, content) turns into new_attacher => {
				let search_offset = if offset > 0 { offset - 1 } else { 0 };
				offset = 0;
				lex!{
					simplex(search_offset)
					unless Simplex(concept) turns into new_simplex => {
						if offset > 0 { offset -= 1; }
						let search_offset = offset;
						offset = original_offset;
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

// #[cfg(test)]
// mod t {
// 	use alloc::vec::Vec;
// 	use super::{Token, any};

// 	macro_rules! test_any {
// 		($sample:literal $expected_info:expr) => {
// 			test_any!{
// 				sample: $sample,
// 				tab_count: 0,
// 				info: $expected_info
// 			}
// 		};
// 		(
// 			sample: $sample:expr,
// 			tab_count: $tab_count:literal,
// 			info: $expected_info:expr
// 		) => {
// 			let info = any(
// 				$sample,
// 				0,
// 				$tab_count
// 			);
// 			assert_eq!(info, $expected_info);
// 		};
// 	}

// 	#[test]
// 	fn can_lex_line_comment() {
// 		test_any!(b"#abc" (Token::LineComment(b"abc"), 4));
// 	}

// 	#[test]
// 	fn can_lex_block_comment() {
// 		let mut expected_lines = Vec::new();
// 		expected_lines.push(&b"\tde"[..]);

// 		test_any!(b"###\n\tde\n###" (Token::BlockComment(expected_lines), 11));
// 	}

// 	#[test]
// 	fn can_lex_simplex() {
// 		test_any!(b"efg|" (Token::Simplex(b"efg"), 4));
// 	}

// 	#[test]
// 	fn can_lex_complex() {
// 		test_any!(b"hi" (Token::Complex(b"hi"), 2));
// 	}

// 	#[test]
// 	fn can_lex_attacher() {
// 		test_any!(b"jklm:\tn" (Token::Attacher(b"jklm", b"n"), 7));
// 	}

// 	#[test]
// 	fn can_lex_line_othertongue() {
// 		test_any!(b"= o" (Token::LineOthertongue(b"o"), 3));
// 	}

// 	#[test]
// 	fn can_lex_block_othertongue() {
// 		let mut expected_lines = Vec::new();
// 		expected_lines.push(&b"pqrs"[..]);

// 		test_any!(b"===\npqrs\n===" (Token::BlockOthertongue(expected_lines), 12));
// 	}

// 	#[test]
// 	fn can_lex_inlined_line_othertongue() {
// 		test_any!(b" = tu" (Token::LineOthertongue(b"tu"), 5));
// 	}
// }
