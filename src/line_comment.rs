use crate::raw_token::{RawToken, RawTokenInfo};
use crate::special_characters::POUND_SIGN;
use crate::find_line_ending;

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
/// use chearmyp_lexer::line_comment;
/// use chearmyp_lexer::RawToken;
///
/// let non_terminated = b"# hello world";
/// let (comment, last_index) = line_comment(&non_terminated[..], 0);
/// if let RawToken::LineComment(comment) = comment {
/// 	assert_eq!(comment, &non_terminated[1..]);
/// } else {
/// 	panic!("The returned raw token is not line comment.");
/// }
/// assert_eq!(last_index, 13);
///
/// let terminated = b"# hello world\n ";
/// let (comment, last_index) = line_comment(&terminated[..], 0);
/// if let RawToken::LineComment(comment) = comment {
/// 	assert_eq!(comment, &terminated[1..13]);
/// } else {
/// 	panic!("The returned raw token is not line comment.");
/// }
/// assert_eq!(last_index, 13);
///
/// let non_comment = b"hello world";
/// let (non_comment, last_index) = line_comment(&non_comment[..], 0);
/// if let RawToken::Invalid = non_comment {
/// 	assert!(true);
/// } else {
/// 	panic!("The returned raw token is not invalid.");
/// }
/// assert_eq!(last_index, 0);
/// ```
pub fn line_comment(src: &[u8], mut i: usize) -> RawTokenInfo {
	let first_character = src.get(i);
	if let Some(&POUND_SIGN) = first_character {
		i += 1;
		let end = find_line_ending(src, i);
		(RawToken::LineComment(&src[i..end]), end)
	} else if let Some(_) = first_character {
		(RawToken::Invalid, i)
	} else {
		(RawToken::Empty, i)
	}
}

#[cfg(test)]
mod t {
	use super::{RawToken, line_comment};

	macro_rules! test_line_comment {
		($sample:literal 0 $variant:ident) => {
			let (raw_token, line_comment_size) = line_comment($sample, 0);
			assert_eq!(line_comment_size, 0);
			assert_eq!(raw_token, RawToken::$variant);
		};
		($sample:literal $expected_size:literal $expected_token:expr) => {
			let (raw_token, line_comment_size) = line_comment($sample, 0);
			assert_eq!(raw_token, RawToken::LineComment(&$expected_token[..]),
				"Expected raw_token of {:?}", $sample);
			assert_eq!(line_comment_size, $expected_size, "Expected length of {:?}", $sample);
		};
	}

	#[test]
	fn can_lex() {
		test_line_comment!(b"#\n" 1 b"");
		test_line_comment!(b"#" 1 b"");
		test_line_comment!(b"# hello" 7 b" hello");
		test_line_comment!(b"# hi\n" 4 b" hi");
	}

	#[test]
	fn cannot_lex() {
		test_line_comment!(b"" 0 Empty);
		test_line_comment!(b"\n" 0 Invalid);
	}
}
