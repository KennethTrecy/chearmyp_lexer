use crate::abstracts::{AbstractSource, ComparableAbstractSource, AbstractBoundary};
use crate::delimeter::Delimeter;
use crate::helpers::find_line_ending;
use crate::raw_token::{RawToken, RawTokenInfo};
use crate::special_characters::EQUAL_THEN_SPACE;

/// Returns the info of recognized line othertogue and the probably last index that has been checked
/// from the source.
///
/// It needs an array of bytes as the first argument (known as source), and where to start looking
/// for the line othertongue (inlined or not) as the second argument (known as the offset).
///
/// ## Notes
/// If there is no valid raw token found, it will return invalid raw token along with the probably
/// last index checked.
///
/// ## Examples
/// ```
/// use std::ops::Range;
/// use chearmyp_lexer::primary_lexers::line_othertongue;
/// use chearmyp_lexer::RawToken;
///
/// let non_terminated = b"= hello world";
/// let (raw_token, last_index) = line_othertongue
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&non_terminated[..], 0);
/// assert_eq!(raw_token, RawToken::LineOthertongue(2..13));
/// assert_eq!(last_index, 13);
///
/// let previous_inlined_yet_terminated = b" = hello world\n";
/// let (raw_token, last_index) = line_othertongue
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&previous_inlined_yet_terminated[..], 0);
/// assert_eq!(raw_token, RawToken::Invalid);
/// assert_eq!(last_index, 0);
/// ```
pub fn line_othertongue<T, U, V>(src: T, offset: usize) -> RawTokenInfo<U, V>
where
	T: AbstractSource + ComparableAbstractSource<&'static str>,
	U: AbstractBoundary<usize> {
	match determine_othertongue_prefix(&src, offset) {
		Delimeter::Pad => {
			let start = offset + 2;
			let end = find_line_ending(&src, start);
			(RawToken::LineOthertongue(U::new(start, end)), end)
		},
		_ => (RawToken::Invalid, offset)
	}
}

pub fn determine_othertongue_prefix<T>(src: &T, offset: usize) -> Delimeter
where
	T: AbstractSource + ComparableAbstractSource<&'static str> {
	if src.is_same_needle_at(offset, EQUAL_THEN_SPACE) {
		Delimeter::Pad
	} else {
		Delimeter::Invalid
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec};
	use super::{RawToken, line_othertongue};

	macro_rules! test_line_othertongue {
		($sample:literal 0 $variant:ident) => {
			let (
				raw_token,
				last_seen_offset
			) = line_othertongue::<&[u8], Range<usize>, Vec<Range<usize>>>(&&$sample[..], 0);
			assert_eq!(last_seen_offset, 0, "Expected raw_token of {:?}", $sample);
			assert_eq!(raw_token, RawToken::$variant, "Expected last seen offset of {:?}", $sample);
		};
		($sample:literal $expected_offset:literal $expected_token:expr) => {
			let (
				raw_token,
				last_seen_offset
			) = line_othertongue::<&[u8], Range<usize>, Vec<Range<usize>>>(&&$sample[..], 0);
			assert_eq!(raw_token, RawToken::LineOthertongue($expected_token),
				"Expected raw_token of {:?}", $sample);
			assert_eq!(last_seen_offset, $expected_offset,
				"Expected last seen offset of {:?}", $sample);
		};
	}

	#[test]
	fn can_lex() {
		test_line_othertongue!(b"= a" 3 2..3);
	}

	#[test]
	fn cannot_lex() {
		test_line_othertongue!(b" = bc" 0 Invalid);
		test_line_othertongue!(b"=d" 0 Invalid);
		test_line_othertongue!(b" =e" 0 Invalid);
		test_line_othertongue!(b"f" 0 Invalid);
	}
}
