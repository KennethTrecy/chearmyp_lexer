macro_rules! create_block {
	($variant:ident $($raw_token:expr),*) => {
		RawToken::$variant(vec![$($raw_token,)*])
	};
}

macro_rules! test_block_lexer {
	(
		lexer: $lexer:ident,
		sample: $sample:expr,
		tab count: $tab_count:literal,
		expected consumed size: $expected_consumed_size:expr,
		expected raw_token: $expected_token:expr
	) => {
		let (raw_token, block_size) = $lexer::<
			&[u8],
			Range<usize>,
			Vec<Range<usize>>
		>($sample, 0, $tab_count);
		assert_eq!(block_size, $expected_consumed_size, "Consumed size of {:?}", $sample);
		assert_eq!(raw_token, $expected_token, "Expected raw_token of {:?}", $sample);
	};
}

macro_rules! test_block_cases {
	(
		lexer: $lexer:ident
		raw_token creator: $token_creator:ident

		valid cases: [
			$(
				$can_test_name:ident with sample $test_sample:literal and tab count $tab_count:literal
				expecting [$($expected_token_contents:expr),*]
				with consumed size of $expected_consumed_size:literal bytes.
			)+
		]

		invalid cases: [
			$(
				$cannot_test_name:ident with sample $cannot_test_sample:literal
				expecting $expected_token_variant:ident.
			)+
		]
	) => {
		$(
			#[test]
			fn $can_test_name() {
				test_block_lexer!{
					lexer: $lexer,
					sample: &&$test_sample[..],
					tab count: $tab_count,
					expected consumed size: $expected_consumed_size,
					expected raw_token: $token_creator![$($expected_token_contents),*]
				}
			}
		)+

		$(
			#[test]
			fn $cannot_test_name() {
				assert_eq!{
					$lexer::<
						&[u8],
						Range<usize>,
						Vec<Range<usize>>
					>(&&$cannot_test_sample[..], 0, 0).0,
					RawToken::$expected_token_variant
				};
			}
		)+
	}
}
