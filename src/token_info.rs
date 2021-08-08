/// Contains the extracted token and its last index occupied in the source.
/// This token is used as return value for some lexers.
pub type TokenInfo<T> = (T, usize);
