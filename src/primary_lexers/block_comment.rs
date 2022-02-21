use crate::abstracts::{
	AbstractSource,
	AbstractBoundary,
	ComparableAbstractSource,
	AbstractBoundaryCollection
};
use crate::helpers::block;
use crate::special_characters::POUND_SIGN;
use crate::raw_token::{RawToken, RawTokenInfo};

/// Returns the info of recognized block comment and its probably last seen index in the source.
///
/// It needs an array of bytes as the first argument (known as source), where to start looking for
/// the pound signs as the second argument (known as the offset), and the number of tabs must the
/// terminating pound signs be indented.
///
/// ## Notes
/// If the source has no 3 pound signs found at the offset, it will return an invalid raw token
/// variant with the offset.
///
/// ## Examples
/// ```
/// use std::ops::Range;
/// use chearmyp_lexer::primary_lexers::block_comment;
/// use chearmyp_lexer::RawToken;
///
/// let terminated = b"###\n\thello world\n###";
/// let (raw_token, last_index) = block_comment
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&terminated[..], 0, 0);
/// assert_eq!(raw_token, RawToken::BlockComment(vec![4..16]));
/// assert_eq!(last_index, 20);
///
/// let non_comment = b"hello world";
/// let (raw_token, last_index) = block_comment
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&non_comment[..], 0, 0);
/// assert_eq!(raw_token, RawToken::Invalid);
/// assert_eq!(last_index, 0);
/// ```
pub fn block_comment<T, U, V>(src: T, offset: usize, tab_count: usize) -> RawTokenInfo<U, V>
where
	T: AbstractSource + ComparableAbstractSource<&'static str> + Clone,
	U: AbstractBoundary<usize>,
	V: AbstractBoundaryCollection<usize, U> {
	let block = block(src, offset, tab_count, POUND_SIGN);
	if let (RawToken::Block(lines), offset) = block {
		(RawToken::BlockComment(lines), offset)
	} else {
		block
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec};
	use super::{RawToken, block_comment};

	macro_rules! BlockComment {
		($($raw_token:expr),*) => {
			create_block!(BlockComment $($raw_token),*)
		};
	}

	test_block_cases!{
		lexer: block_comment
		raw_token creator: BlockComment

		valid cases: [
			can_lex_empty_comment with sample b"###\n###" and tab count 0
			expecting [7..7] with consumed size of 7 bytes.

			can_lex_comment_with_unindented_line
			with sample b"###\nhello world!\n###" and tab count 0
			expecting [4..16] with consumed size of 20 bytes.

			can_lex_comment_with_indented_line
			with sample b"###\n\thello world!\n\t###" and tab count 1
			expecting [4..17] with consumed size of 22 bytes.

			can_lex_comment_with_indented_lines
			with sample b"###\n\thello world!\n\t\thi universe\n\t\t###" and tab count 2
			expecting [4..17, 18..31] with consumed size of 37 bytes.

			can_lex_comment_with_empty_line
			with sample b"###\n\n\thello world\n\t###" and tab count 1
			expecting [4..4, 5..17] with consumed size of 22 bytes.

			can_lex_comment_with_empty_lines
			with sample b"###\n\n\n\n###" and tab count 0
			expecting [4..4, 5..5, 6..6] with consumed size of 10 bytes.

			can_lex_comment_with_empty_line_and_indented_line
			with sample b"###\n\t\thello world!\n\nhi universe\n\t###" and tab count 1
			expecting [4..18, 19..19, 20..31] with consumed size of 36 bytes.
		]

		invalid cases: [
			cannot_lex_empty_string with sample b"" expecting Empty.
			cannot_lex_single_pound_sign with sample b"#" expecting Invalid.
			cannot_lex_double_pound_sign with sample b"##" expecting Invalid.
		]
	}
}
