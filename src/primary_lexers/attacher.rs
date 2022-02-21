use crate::abstracts::{AbstractSource, ComparableAbstractSource, AbstractBoundary};
use crate::delimeter::Delimeter;
use crate::raw_token::{RawToken, RawTokenInfo};
use crate::special_characters::{COLON, NEW_LINE, SPACE, TAB};

/// Returns the info of recognized attacher and the last index that has been checked from the
/// source.
///
/// It needs an array of bytes as the first argument (known as source), where to start slicing
/// (known as slice offset) as the second argument, and where to start looking for the terminator
/// (such as tab, new line, or equal sign of the inlined othertongue) as the third argument (known
/// as the search offset).
///
/// ## Notes
/// If there is no valid raw token found, it will return invalid raw token along with the last index
/// checked.
///
/// ## Examples
/// ```
/// use std::ops::Range;
/// use chearmyp_lexer::primary_lexers::attacher;
/// use chearmyp_lexer::RawToken;
///
/// let non_terminated = b"hello:	world";
/// let (raw_token, last_index) = attacher
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&non_terminated[..], 0, 0);
/// assert_eq!(raw_token, RawToken::Attacher(0..5, 7..12));
/// assert_eq!(last_index, 12);
///
/// let terminated = b"hello:	world\n";
/// let (raw_token, last_index) = attacher
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&terminated[..], 0, 0);
/// assert_eq!(raw_token, RawToken::Attacher(0..5, 7..12));
/// assert_eq!(last_index, 12);
///
/// let simplex = b"hello world";
/// let (raw_token, last_index) = attacher
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&simplex[..], 0, 0);
/// assert_eq!(raw_token, RawToken::Invalid);
/// assert_eq!(last_index, 11);
/// ```
pub fn attacher<T, U, V>(src: T, slice_offset: usize, mut search_offset: usize)
-> RawTokenInfo<U, V>
where
	T: AbstractSource + ComparableAbstractSource<&'static str>,
	U: AbstractBoundary<usize> {
	let label_start = slice_offset;
	let label_end;

	loop {
		let separator = determine_separator(&src, search_offset);
		match separator {
			Delimeter::Incorrect => search_offset += 1,
			Delimeter::Pad => {
				label_end = search_offset;
				search_offset += 1;
				break;
			},
			_ => return (RawToken::Invalid, search_offset)
		}
	}

	let label = U::new(label_start, label_end);

	loop {
		if src.is_same_needle_at(search_offset, TAB) || src.is_same_needle_at(search_offset, SPACE) {
			search_offset += 1;
		} else if src.is_empty_at(search_offset) {
			return (RawToken::Invalid, search_offset)
		} else {
			break;
		}
	}

	let content_start = search_offset;
	let content_end;

	loop {
		let ending = determine_ending(&src, search_offset);
		match ending {
			Delimeter::Incorrect => search_offset += 1,
			Delimeter::Pad | Delimeter::Limit => {
				content_end = search_offset;
				break;
			},
			Delimeter::Invalid => return (RawToken::Invalid, search_offset)
		}
	}

	let content = U::new(content_start, content_end);
	(RawToken::Attacher(label, content), search_offset)
}

fn determine_separator<T>(src: &T, offset: usize) -> Delimeter
where
	T: AbstractSource + ComparableAbstractSource<&'static str> {
	if src.is_same_needle_at(offset, COLON) {
		let next_offset = offset + 1;
		if src.is_same_needle_at(next_offset, TAB) || src.is_same_needle_at(next_offset, SPACE) {
			Delimeter::Pad
		} else if src.is_same_needle_at(next_offset, NEW_LINE) || src.is_empty_at(next_offset){
			Delimeter::Invalid
		} else {
			Delimeter::Incorrect
		}
	} else if src.is_same_needle_at(offset, NEW_LINE) || src.is_same_needle_at(offset, TAB) {
		Delimeter::Invalid
	} else if src.is_empty_at(offset) {
		Delimeter::Limit
	} else {
		Delimeter::Incorrect
	}
}

fn determine_ending<T>(src: &T, offset: usize) -> Delimeter
where
	T: AbstractSource + ComparableAbstractSource<&'static str> {
	if src.is_same_needle_at(offset, NEW_LINE) || src.is_same_needle_at(offset, TAB) {
		Delimeter::Pad
	} else if src.is_empty_at(offset) {
		Delimeter::Limit
	} else {
		Delimeter::Incorrect
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec};
	use super::{RawToken, attacher};

	macro_rules! test_attacher {
		(
			$sample:literal,
			$expected_token:expr,
			$expected_consumption:literal
		) => {
			let (raw_token, consumed_size) = attacher
				::<&[u8], Range<usize>, Vec<Range<usize>>>(&&$sample[..], 0, 0);
			assert_eq!(raw_token, $expected_token);
			assert_eq!(consumed_size, $expected_consumption);
		};
	}

	macro_rules! Attacher {
		($label:expr, $content:expr) => {
			RawToken::Attacher($label, $content)
		};
	}

	#[test]
	fn can_lex() {
		test_attacher!(b"a:	b", Attacher!(0..1, 3..4), 4);
		test_attacher!(b"cd:		e", Attacher!(0..2, 5..6), 6);
		test_attacher!(b"f:		g\n", Attacher!(0..1, 4..5), 5);
		test_attacher!(b"h:	i	j:	k", Attacher!(0..1, 3..4), 4);
	}

	#[test]
	fn cannot_lex() {
		test_attacher!(b"lm", RawToken::Invalid, 2);
		test_attacher!(b"n|", RawToken::Invalid, 2);
		test_attacher!(b"o:	", RawToken::Invalid, 3);
	}

	#[test]
	fn can_lex_separated_by_colon_then_space() {
		test_attacher!(b"p: q", Attacher!(0..1, 3..4), 4);
	}
}
