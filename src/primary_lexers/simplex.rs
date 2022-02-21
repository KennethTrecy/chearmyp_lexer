use crate::abstracts::{AbstractSource, ComparableAbstractSource, AbstractBoundary};
use crate::delimeter::Delimeter;
use crate::raw_token::{RawToken, RawTokenInfo};
use crate::special_characters::{NEW_LINE, TAB, VERTICAL_LINE};

/// Returns the info of recognized simplex and the last index that has been checked from the source.
///
/// It needs an array of bytes as the first argument (known as source), where to start slicing
/// (known as slice offset) as the second argument, and where to start looking for the vertical line
/// as the third argument (known as the search offset).
///
/// ## Notes
/// It will return invalid raw token if there is no vertical line from the specified offset in
/// source. Also, it does not differentiate attachers because there may be a case where the content
/// of an attacher ends in vertical line. Use [`attacher()`] lexer first.
///
/// ## Examples
/// ```
/// use std::ops::Range;
/// use chearmyp_lexer::primary_lexers::simplex;
/// use chearmyp_lexer::RawToken;
///
/// let terminated = b"hello world|";
/// let (raw_token, last_index) = simplex
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&terminated[..], 0, 0);
/// assert_eq!(raw_token, RawToken::Simplex(0..11));
/// assert_eq!(last_index, 12);
///
/// let non_simplex = b"hello world";
/// let (raw_token, last_index) = simplex
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&non_simplex[..], 0, 0);
/// assert_eq!(raw_token, RawToken::Invalid);
/// assert_eq!(last_index, 11);
/// ```
///
/// [`attacher()`]: ./fn.attacher.html
pub fn simplex<T, U, V>(src: T, slice_offset: usize, mut search_offset: usize)
-> RawTokenInfo<U, V>
where
	T: AbstractSource + ComparableAbstractSource<&'static str>,
	U: AbstractBoundary<usize> {
	let start = slice_offset;
	let end;

	loop {
		let ending = determine_ending(&src, search_offset);
		match ending {
			Delimeter::Incorrect => search_offset += 1,
			Delimeter::Invalid => { return (RawToken::Invalid, search_offset); },
			Delimeter::Pad | Delimeter::Limit => {
				end = search_offset;
				search_offset += 1;
				break;
			}
		}
	}

	(RawToken::Simplex(U::new(start, end)), search_offset)
}

fn determine_ending<T>(src: &T, offset: usize) -> Delimeter
where
	T: AbstractSource + ComparableAbstractSource<&'static str> {
	if src.is_same_needle_at(offset, VERTICAL_LINE) {
		let next_offset = offset + 1;
		if src.is_same_needle_at(next_offset, NEW_LINE) || src.is_same_needle_at(next_offset, TAB) {
			Delimeter::Pad
		} else if src.is_empty_at(next_offset) {
			Delimeter::Limit
		} else {
			Delimeter::Incorrect
		}
	} else if src.is_same_needle_at(offset, NEW_LINE) || src.is_same_needle_at(offset, TAB) {
		Delimeter::Invalid
	} else if src.is_empty_at(offset) {
		Delimeter::Invalid
	} else {
		Delimeter::Incorrect
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec};
	use super::{RawToken, simplex};

	macro_rules! test_simplex {
		(
			$sample:literal,
			$expected_token:expr,
			$expected_consumption:literal
		) => {
			let (raw_token, consumed_size) = simplex::<&[u8], Range<usize>, Vec<Range<usize>>>(&&&$sample[..], 0, 0);
			assert_eq!(raw_token, $expected_token);
			assert_eq!(consumed_size, $expected_consumption);
		};
	}

	#[test]
	fn can_lex() {
		test_simplex!(b"a|	", RawToken::Simplex(0..1), 2);
		test_simplex!(b"bc|	#", RawToken::Simplex(0..2), 3);
		test_simplex!(b"def|\n#", RawToken::Simplex(0..3), 4);
		test_simplex!(b"kl|", RawToken::Simplex(0..2), 3);
	}

	#[test]
	fn cannot_lex() {
		test_simplex!(b"g\n", RawToken::Invalid, 1);
		test_simplex!(b"hi\tj", RawToken::Invalid, 2);
		test_simplex!(b"mn", RawToken::Invalid, 2);
		test_simplex!(b"o: pq", RawToken::Invalid, 5);
	}
}
