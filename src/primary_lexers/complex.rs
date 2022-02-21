use crate::abstracts::{AbstractSource, ComparableAbstractSource, AbstractBoundary};
use crate::delimeter::Delimeter;
use crate::raw_token::{RawToken, RawTokenInfo};
use crate::special_characters::{NEW_LINE, TAB};

/// Returns the info of recognized complex and the last index that has been checked from the source.
///
/// It needs an array of bytes as the first argument (known as source), where to start slicing
/// (known as slice offset) as the second argument, and where to start looking for the terminator
/// (such as tab, new line, or equal sign of the inlined othertongue) as the third argument (known
/// as the search offset).
///
/// ## Notes
/// This lexer does not differentiate simplexes and attachers. Use [`simplex()`] and [`attacher()`]
/// lexers first.
///
/// ## Examples
/// ```
/// use std::ops::Range;
/// use chearmyp_lexer::primary_lexers::complex;
/// use chearmyp_lexer::RawToken;
///
/// let non_terminated = b"hello world";
/// let (raw_token, last_index) = complex
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&non_terminated[..], 0, 0);
/// assert_eq!(raw_token, RawToken::Complex(0..11));
/// assert_eq!(last_index, 11);
///
/// let terminated = b"hello world\n";
/// let (raw_token, last_index) = complex
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&terminated[..], 0, 0);
/// assert_eq!(raw_token, RawToken::Complex(0..11));
/// assert_eq!(last_index, 11);
///
/// // Does not differentiate simplexes and attachers.
/// let simplex = b"hello world|";
/// let (raw_token, last_index) = complex
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&simplex[..], 0, 0);
/// assert_eq!(raw_token, RawToken::Complex(0..12));
/// assert_eq!(last_index, 12);
/// ```
///
/// [`simplex()`]: ./fn.simplex.html
/// [`attacher()`]: ./fn.attacher.html
pub fn complex<T, U, V>(src: T, slice_offset: usize, mut search_offset: usize)
-> RawTokenInfo<U, V>
where
	T: AbstractSource + ComparableAbstractSource<&'static str>,
	U: AbstractBoundary<usize> {
	let slice_end;

	loop {
		let ending = determine_ending(&src, search_offset);
		match ending {
			Delimeter::Incorrect => search_offset += 1,
			Delimeter::Pad | Delimeter::Limit => {
				slice_end = search_offset;
				break;
			},
			Delimeter::Invalid => return (RawToken::Invalid, search_offset)
		}
	}

	(RawToken::Complex(U::new(slice_offset, slice_end)), search_offset)
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
	use super::{RawToken, complex};

	macro_rules! test_complex {
		($sample:literal, $expected_token:expr, $expected_consumption:literal) => {
			let (raw_token, consumed_size) = complex
				::<&[u8], Range<usize>, Vec<Range<usize>>>($sample, 0, 0);
			assert_eq!(raw_token, $expected_token);
			assert_eq!(consumed_size, $expected_consumption);
		};
	}

	macro_rules! Complex {
		($raw_token:expr) => {
			RawToken::Complex($raw_token)
		};
	}

	#[test]
	fn can_lex() {
		test_complex!(b"a", Complex!(0..1), 1);
		test_complex!(b"bc	", Complex!(0..2), 2);
		test_complex!(b"d\n", Complex!(0..1), 1);
		test_complex!(b"e = f\n", Complex!(0..5), 5);
	}
}
