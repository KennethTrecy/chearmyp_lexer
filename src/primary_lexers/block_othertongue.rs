use crate::abstracts::{
	AbstractSource,
	AbstractBoundary,
	ComparableAbstractSource,
	AbstractBoundaryCollection
};
use crate::helpers::block;
use crate::special_characters::EQUAL;
use crate::raw_token::{RawToken, RawTokenInfo};

/// Returns the info of recognized block othertongue and its probably last seen index in the source.
///
/// It needs an array of bytes as the first argument (known as source), where to start looking for
/// the equal signs as the second argument (known as the offset), and the number of tabs must the
/// terminating equal signs be indented.
///
/// ## Notes
/// If the source has no 3 equal signs found at the offset, it will return an invalid raw token
/// variant with the offset.
///
/// ## Examples
/// ```
/// use std::ops::Range;
/// use chearmyp_lexer::primary_lexers::block_othertongue;
/// use chearmyp_lexer::RawToken;
///
/// let terminated = b"===\n\thello world\n===\n";
/// let (raw_token, last_index) = block_othertongue
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&terminated[..], 0, 0);
/// assert_eq!(raw_token, RawToken::BlockOthertongue(vec![4..16]));
/// assert_eq!(last_index, 21);
///
/// let non_othertongue = b"hello world";
/// let (raw_token, last_index) = block_othertongue
/// 	::<&[u8], Range<usize>, Vec<Range<usize>>>(&non_othertongue[..], 0, 0);
/// assert_eq!(raw_token, RawToken::Invalid);
/// assert_eq!(last_index, 0);
/// ```
pub fn block_othertongue<T, U, V>(src: T, offset: usize, tab_count: usize) -> RawTokenInfo<U, V>
where
	T: AbstractSource + ComparableAbstractSource<&'static str> + Clone,
	U: AbstractBoundary<usize>,
	V: AbstractBoundaryCollection<usize, U> {
	let block = block(src, offset, tab_count, EQUAL);
	if let (RawToken::Block(lines), offset) = block {
		(RawToken::BlockOthertongue(lines), offset)
	} else {
		block
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec};
	use super::{RawToken, block_othertongue};

	macro_rules! BlockOthertongue {
		($($raw_token:expr),*) => {
			create_block!(BlockOthertongue $($raw_token),*)
		};
	}

	test_block_cases!{
		lexer: block_othertongue
		raw_token creator: BlockOthertongue

		valid cases: [
			can_lex_empty_othertongue
			with sample b"===\n===" and tab count 0
			expecting [7..7] with consumed size of 7 bytes.

			can_lex_othertongue_with_single_line
			with sample b"===\na\n===" and tab count 0
			expecting [4..5] with consumed size of 9 bytes.

			can_lex_othertongue_with_indented_and_single_line
			with sample b"===\n\tbc\n\t===" and tab count 1
			expecting [4..7] with consumed size of 12 bytes.

			can_lex_othertongue_with_multiple_indented_lines
			with sample b"===\n\td\n\t\te\n\t\t===" and tab count 2
			expecting [4..6, 7..10] with consumed size of 16 bytes.

			can_lex_othertongue_with_empty_line
			with sample b"===\nf\n\n===" and tab count 0
			expecting [4..5, 6..6] with consumed size of 10 bytes.

			can_lex_othertongue_with_empty_lines
			with sample b"===\n\n\n\n\n\t===" and tab count 1
			expecting [4..4, 5..5, 6..6, 7..7] with consumed size of 12 bytes.

			can_lex_othertongue_with_empty_line_and_indented_line
			with sample b"===\n\tg\n\nh\n\t===" and tab count 1
			 expecting [4..6, 7..7, 8..9] with consumed size of 14 bytes.
		]

		invalid cases: [
			cannot_lex_on_empty_line with sample b"" expecting Empty.
			cannot_lex_on_single_character_line with sample b"=" expecting Invalid.
			cannot_lex_on_double_character_line with sample b"==" expecting Invalid.
		]
	}
}
