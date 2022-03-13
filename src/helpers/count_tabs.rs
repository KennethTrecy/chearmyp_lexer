use crate::abstracts::{AbstractSource, ComparableAbstractSource};
use crate::special_characters::TAB;

/// Returns the number of initial tabs in the source.
///
/// It needs an array of bytes as the first argument (known as source), and the previous number of
/// tabs worked on (known as old tab count). If it is the first time to check the number of initial
/// tabs, set the old tab count to 0.
pub fn count_tabs<T>(src: T, old_tab_count: usize) -> usize
where
	T: AbstractSource + ComparableAbstractSource<&'static str> {
	let mut new_tab_count = old_tab_count;

	loop {
		if src.is_same_needle_at(new_tab_count, TAB) {
			new_tab_count += 1;
		} else if src.is_empty_at(new_tab_count) {
			if old_tab_count == new_tab_count {
				new_tab_count = 0;
			}
			break;
		} else {
			if new_tab_count > 0 {
				if src.is_same_needle_at(new_tab_count - 1, TAB) {
					break;
				} else {
					new_tab_count -= 1;
				}
			} else {
				break;
			}
		}
	}

	new_tab_count
}

#[cfg(test)]
mod t {
	use super::count_tabs;

	#[test]
	fn can_count_on_first_time() {
		let sample = b"a";
		let old_tab_count = 0;
		let expected_new_tab_count = 0;

		let count = count_tabs(&sample[..], old_tab_count);

		assert_eq!(count, expected_new_tab_count);
	}

	#[test]
	fn can_increase_count_on_first_time() {
		let sample = b"\t";
		let old_tab_count = 0;
		let expected_new_tab_count = 1;

		let count = count_tabs(&sample[..], old_tab_count);

		assert_eq!(count, expected_new_tab_count);
	}

	#[test]
	fn can_count_decreased_tabs() {
		let sample = b"bcd";
		let old_tab_count = 3;
		let expected_new_tab_count = 0;

		let count = count_tabs(&sample[..], old_tab_count);

		assert_eq!(count, expected_new_tab_count);
	}

	#[test]
	fn can_count_remain_tab_count() {
		let sample = b"\te";
		let old_tab_count = 1;
		let expected_new_tab_count = 1;

		let count = count_tabs(&sample[..], old_tab_count);

		assert_eq!(count, expected_new_tab_count);
	}

	#[test]
	fn can_count_increased_tabs() {
		let sample = b"\t\tfg";
		let old_tab_count = 1;
		let expected_new_tab_count = 2;

		let count = count_tabs(&sample[..], old_tab_count);

		assert_eq!(count, expected_new_tab_count);
	}

	#[test]
	fn can_count_dramatically_increased_tabs() {
		let sample = b"\t\t\tfg";
		let old_tab_count = 0;
		let expected_new_tab_count = 3;

		let count = count_tabs(&sample[..], old_tab_count);

		assert_eq!(count, expected_new_tab_count);
	}
}
