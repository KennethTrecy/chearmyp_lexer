use crate::abstracts::{
	AbstractSource,
	AbstractBoundary,
	ComparableAbstractSource,
	AbstractBoundaryCollection
};
use crate::helpers::find_line_ending;
use crate::raw_token::{RawToken, RawTokenInfo};
use crate::special_characters::{NEW_LINE, TAB};

/// Returns the recognized block and the last seen index.
///
/// This is a generalization of blocks in chearmyp. It will return a vector of lines that are in the
/// block.
///
/// ## Example
/// ```
/// use chearmyp_lexer::RawToken;
/// use chearmyp_lexer::helpers::block;
///
/// let special_character = "@";
/// let sample_block = b"
/// @@@
/// hello world
/// @@@";
/// let (block, last_seen_index) = block(&sample_block[..], 1, 0, special_character);
/// assert_eq!(block, RawToken::Block(vec![5..16]));
/// assert_eq!(last_seen_index, 20);
/// ```
pub fn block<T, U, V>(src: T, offset: usize, tab_count: usize, special_character: &'static str)
-> RawTokenInfo<U, V>
where
	T: AbstractSource + ComparableAbstractSource<&'static str> + Clone,
	U: AbstractBoundary<usize>,
	V: AbstractBoundaryCollection<usize, U> {
	let has_special_characters = has_3_special_characters(&src, offset, special_character);
	if has_special_characters {
		let mut lines = None;
		let mut offset = offset + 3;
		offset += if src.is_same_needle_at(offset, NEW_LINE) { 1 } else { 0 };

		loop {
			let start = offset;
			let end = find_line_ending(&src, start);
			if start == end && src.is_empty_at(end) { break; }
			let line = src.clone().slice(start, end);

			let mut indent_size = tab_count;
			while indent_size > 0 {
				indent_size -= 1;
				if !line.is_same_needle_at(indent_size, TAB) { break; }
			}

			offset = end;

			if indent_size == 0 && has_3_special_characters(&line, tab_count, special_character) {
				if src.is_same_needle_at(offset, NEW_LINE) { offset += 1; }
				break;
			}

			offset += 1;
			lines = lines.map(|mut lines: V| {
				lines.add(U::new(start, end));
				lines
			}).or_else(|| {
				Some(V::new(start, end))
			});
		}

		lines = lines.or_else(|| {
			Some(V::new(offset, offset))
		});

		(RawToken::Block(lines.unwrap()), offset)
	} else {
		let raw_token = if src.is_empty_at(offset) { RawToken::Empty } else { RawToken::Invalid };
		(raw_token, offset)
	}
}

fn has_3_special_characters<T>(src: &T, offset: usize, special_character: &'static str) -> bool
where
	T: AbstractSource + ComparableAbstractSource<&'static str> {
	if src.is_empty_at(offset + 2) {
		false
	} else {
		src.is_same_needle_at(offset, special_character)
		&& src.is_same_needle_at(offset + 1, special_character)
		&& src.is_same_needle_at(offset + 2, special_character)
	}
}

#[cfg(test)]
mod t {
	use crate::native::{Range, Vec};
	use super::{RawToken, has_3_special_characters, block};

	macro_rules! has_3_special_characters {
		($src:literal $offset:literal $special_character:literal) => {
			has_3_special_characters::<&[u8]>(&&$src[..], $offset, $special_character)
		};
	}

	#[test]
	fn can_detect_special_characters() {
		assert!(has_3_special_characters!(b"aaa" 0 "a"), "Normal string");
	}

	#[test]
	fn cannot_detect_special_characters_on_empty_line() {
		assert!(!has_3_special_characters!(b"" 0 "a"), "Empty string");
	}

	#[test]
	fn cannot_detect_special_characters_on_single_character_line() {
		assert!(!has_3_special_characters!(b"a" 0 "a"), "Single-character string");
	}

	#[test]
	fn cannot_detect_special_characters_on_double_character_line() {
		assert!(!has_3_special_characters!(b"aa" 0 "a"), "Double-character string");
	}

	macro_rules! test {
		(
			$(
				$test_name:ident using
					$src:literal,
					$offset:literal,
					$tab_count:literal,
					and $special_character:literal
				expecting $variant_name:ident $(with [$($ranges:expr),+])?
				last seen at $last_seen_index:literal
			)+
		) => {
			$(

				#[test]
				fn $test_name() {
					let source = $src;

					let info = block::<&[u8], Range<usize>, Vec<Range<usize>>>(
						&&source[..],
						$offset,
						$tab_count,
						$special_character);

					assert_eq!{
						info,
						(
							RawToken::$variant_name$((vec![$($ranges),+]))?,
							$last_seen_index
						)
					};
				}
			)+
		};
	}

	test!{
		can_lex_with_proper_content using b"bbb\nc\nbbb", 0, 0, and "b"
		expecting Block with [4..5] last seen at 9

		can_lex_with_an_empty_line using b"ddd\nddd", 0, 0, and "d"
		expecting Block with [7..7] last seen at 7

		can_lex_with_empty_line_and_tabbed_line using b"eee\n \n\t \n\n \n\teee", 0, 1, and "e"
		expecting Block with [4..5, 6..8, 9..9, 10..11] last seen at 16

		can_lex_with_lines_with_fewer_tabs using b"~~~\n\t\t \n\t \n\t\t~~~", 0, 2, and "~"
		expecting Block with [4..7, 8..10] last seen at 16

		cannot_lex_on_empty_line using b"", 0, 0, and "i"
		expecting Empty last seen at 0

		cannot_lex_on_single_character_line using b"i", 0, 0, and "i"
		expecting Invalid last seen at 0

		cannot_lex_on_double_character_line using b"ii", 0, 0, and "i"
		expecting Invalid last seen at 0
	}
}
