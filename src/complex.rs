use crate::raw_token::{RawToken, RawTokenInfo};
use crate::special_characters::{NEW_LINE, SPACE, TAB};
use crate::delimeter::Delimeter;
use crate::line_othertongue::determine_othertongue_prefix;

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
/// use chearmyp_lexer::complex;
/// use chearmyp_lexer::RawToken;
///
/// let non_terminated = b"hello world";
/// let (raw_token, last_index) = complex(&non_terminated[..], 0, 0);
/// if let RawToken::Complex(raw_token) = raw_token {
/// 	assert_eq!(raw_token, &b"hello world"[..]);
/// } else {
/// 	panic!("The returned raw token is not complex.");
/// }
/// assert_eq!(last_index, 11);
///
/// let terminated = b"hello world\n";
/// let (raw_token, last_index) = complex(&terminated[..], 0, 0);
/// if let RawToken::Complex(raw_token) = raw_token {
/// 	assert_eq!(raw_token, &b"hello world"[..]);
/// } else {
/// 	panic!("The returned raw token is not complex.");
/// }
/// assert_eq!(last_index, 11);
///
/// // Does not differentiate simplexes and attachers.
/// let simplex = b"hello world|";
/// let (simplex, last_index) = complex(&simplex[..], 0, 0);
/// if let RawToken::Complex(raw_token) = simplex {
/// 	assert_eq!(raw_token, &b"hello world|"[..]);
/// } else {
/// 	panic!("The returned raw token is not complex.");
/// }
/// assert_eq!(last_index, 12);
/// ```
///
/// [`simplex()`]: ./fn.simplex.html
/// [`attacher()`]: ./fn.attacher.html
pub fn complex(src: &[u8], slice_offset: usize, mut search_offset: usize) -> RawTokenInfo {
	let slice_end;

	loop {
		let ending = determine_ending(src, search_offset);
		match ending {
			Delimeter::Incorrect => search_offset += 1,
			Delimeter::Pad | Delimeter::Limit => {
				slice_end = search_offset;
				break;
			},
			Delimeter::Invalid => return (RawToken::Invalid, search_offset)
		}
	}

	(RawToken::Complex(&src[slice_offset..slice_end]), search_offset)
}

fn determine_ending(src: &[u8], offset: usize) -> Delimeter {
	match src.get(offset) {
		Some(&NEW_LINE) | Some(&TAB) => Delimeter::Pad,
		Some(&SPACE) => {
			if let Delimeter::Pad = determine_othertongue_prefix(src, offset) {
				Delimeter::Pad
			} else {
				Delimeter::Incorrect
			}
		},
		Some(_) => Delimeter::Incorrect,
		None => Delimeter::Limit
	}
}

#[cfg(test)]
mod t {
	use super::{RawToken, complex};

	macro_rules! test_complex {
		($sample:literal, $expected_token:expr, $expected_consumption:literal) => {
			let (raw_token, consumed_size) = complex($sample, 0, 0);
			assert_eq!(raw_token, $expected_token);
			assert_eq!(consumed_size, $expected_consumption);
		};
	}

	macro_rules! Complex {
		($raw_token:literal) => {
			RawToken::Complex(&$raw_token[..])
		};
	}

	#[test]
	fn can_lex() {
		test_complex!(b"a", Complex!(b"a"), 1);
		test_complex!(b"bc	", Complex!(b"bc"), 2);
		test_complex!(b"d\n", Complex!(b"d"), 1);
		test_complex!(b"e = f\n", Complex!(b"e"), 1);
	}
}
