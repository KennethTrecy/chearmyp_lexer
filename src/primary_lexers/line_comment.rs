use crate::abstracts::{AbstractSource, AbstractBoundary, ComparableAbstractSource};
use crate::helpers::find_line_ending;
use crate::special_characters::POUND_SIGN;
use crate::raw_token::{RawToken, RawTokenInfo};

/// Returns the info of recognized line comment and its last index occupied in the source.
///
/// It needs an array of bytes as the first argument (known as source) and where to start looking
/// for the pound sign as the second argument (known as the offset). The raw token will not have a
/// line ending but it will be counted as the last index.
///
/// ## Notes
/// If there is no character at the offset from the source, it will return an empty raw token
/// variant. If the source has no pound sign found at the offset, it will return an invalid raw
/// token variant with the offset.
///
/// ## Examples
/// ```
/// use std::ops::Range;
/// use chearmyp_lexer::primary_lexers::line_comment;
/// use chearmyp_lexer::RawToken;
///
/// let non_terminated = b"# hello world";
/// let (raw_token, last_index) = line_comment
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&non_terminated[..], 0);
/// assert_eq!(raw_token, RawToken::LineComment(1..13));
/// assert_eq!(last_index, 13);
///
/// let terminated = b"# hello world\n ";
/// let (raw_token, last_index) = line_comment
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&terminated[..], 0);
/// assert_eq!(raw_token, RawToken::LineComment(1..13));
/// assert_eq!(last_index, 13);
///
/// let non_comment = b"hello world";
/// let (raw_token, last_index) = line_comment
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&non_comment[..], 0);
/// assert_eq!(raw_token, RawToken::Invalid);
/// assert_eq!(last_index, 0);
/// ```
pub fn line_comment<T, U, V>(src: T, mut i: usize) -> RawTokenInfo<U, V>
where
	T: AbstractSource + ComparableAbstractSource<&'static str>,
	U: AbstractBoundary<usize> {
	if src.is_same_needle_at(i, POUND_SIGN) {
		i += 1;
		let end = find_line_ending(&src, i);
		(RawToken::LineComment(U::new(i, end)), end)
	} else if src.is_empty_at(i) {
		(RawToken::Empty, i)
	} else {
		(RawToken::Invalid, i)
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec};
	use super::{RawToken, line_comment};

	macro_rules! test_line_comment {
		($sample:literal 0 $variant:ident) => {
			let (raw_token, line_comment_size) = line_comment
				::<&[u8], Range<usize>, Vec<Range<usize>>>($sample, 0);
			assert_eq!(line_comment_size, 0);
			assert_eq!(raw_token, RawToken::$variant);
		};
		($sample:literal $expected_size:literal $expected_token:expr) => {
			let (raw_token, line_comment_size) = line_comment
				::<&[u8], Range<usize>, Vec<Range<usize>>>($sample, 0);
			assert_eq!(raw_token, RawToken::LineComment($expected_token),
				"Expected raw_token of {:?}", $sample);
			assert_eq!(line_comment_size, $expected_size, "Expected length of {:?}", $sample);
		};
	}

	#[test]
	fn can_lex() {
		test_line_comment!(b"#\n" 1 1..1);
		test_line_comment!(b"#" 1 1..1);
		test_line_comment!(b"# hello" 7 1..7);
		test_line_comment!(b"# hi\n" 4 1..4);
	}

	#[test]
	fn cannot_lex() {
		test_line_comment!(b"" 0 Empty);
		test_line_comment!(b"\n" 0 Invalid);
	}
}
