macro_rules! create_block {
	($variant:ident $($token:literal)*) => {
		Token::$variant(alloc::vec![$(&$token[..],)*])
	};
}

macro_rules! test_block_lexer {
	(
		lexer: $lexer:ident,
		sample: $sample:expr,
		tab count: $tab_count:literal,
		expected consumed size: $expected_consumed_size:expr,
		expected token: $expected_token:expr
	) => {
		let (token, block_size) = $lexer($sample, 0, $tab_count);
		assert_eq!(block_size, $expected_consumed_size, "Consumed size of {:?}", $sample);
		assert_eq!(token, $expected_token, "Expected token of {:?}", $sample);
	};
}

macro_rules! test_block_cases {
	(
		lexer: $lexer:ident
		token creator: $token_creator:ident

		valid cases: [
			$(
				$can_test_name:ident with sample $test_sample:literal and tab count $tab_count:literal
				expecting [$($expected_token_contents:literal)*]
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
					sample: $test_sample,
					tab count: $tab_count,
					expected consumed size: $expected_consumed_size,
					expected token: $token_creator![$($expected_token_contents)*]
				}
			}
		)+

		$(
			#[test]
			fn $cannot_test_name() {
				assert_eq!($lexer(&$cannot_test_sample[..], 0, 0).0, Token::$expected_token_variant);
			}
		)+
	}
}