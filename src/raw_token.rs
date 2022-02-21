/// Contains the raw tokens used for lexing only.
#[derive(Debug, PartialEq)]
pub enum RawToken<T, U> {
	Empty,
	Invalid,
	ScopeLevel(usize),
	Block(U),
	LineComment(T),
	BlockComment(U),
	Simplex(T),
	Complex(T),
	Attacher(T, T),
	LineOthertongue(T),
	BlockOthertongue(U)
}

/// Contains the extracted raw token and its last index occupied in the source.
/// This raw token is used as return value for most lexers.
pub type RawTokenInfo<U, V> = (RawToken<U, V>, usize);
