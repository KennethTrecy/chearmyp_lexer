/// Contains `count_tabs()` counter.
mod count_tabs;

/// Contains `find_line_ending()`.
mod find_line_ending;

/// Contains `block()` lexer.
mod block;

pub use block::block;
pub use count_tabs::count_tabs;
pub use find_line_ending::find_line_ending;
