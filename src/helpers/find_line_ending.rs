use crate::abstracts::{AbstractSource, ComparableAbstractSource};
use crate::special_characters::NEW_LINE;

/// Returns the index of the first line ending found in the source.
///
/// You can specify where to start looking for the line ending (known as offset). If there is no
/// line ending found from the offset up to the last index, the source's length will be returned.
///
/// ## Examples
/// ```
/// use chearmyp_lexer::helpers::find_line_ending;
///
/// let a = b"hello world";
/// assert_eq!(find_line_ending(&&a[..], 0), 11, "Without line ending");
///
/// let a = b"hello\nworld\n";
/// assert_eq!(find_line_ending(&&a[..], 0), 5, "Unskipped line ending");
/// assert_eq!(find_line_ending(&&a[..], 6), 11, "Skipped line ending through offset");
/// ```
pub fn find_line_ending<T>(src: &T, mut offset: usize)-> usize
where
	T: AbstractSource + ComparableAbstractSource<&'static str> {
	loop {
		if src.is_same_needle_at(offset, NEW_LINE) || src.is_empty_at(offset) {
			break;
		} else {
			offset += 1;
		}
	}

	return offset;
}
