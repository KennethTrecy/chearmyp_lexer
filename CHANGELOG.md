# Changelog

## v0.7.1
- Fix infinite looping when source has empty scopes.
  - This is what causes the large consumption of memory in issue [#1].
- `count_tabs()` now returns 0 as the new tab count for the only indentions in the end of the file.
  - This also prevents infinite looping as stated from above.

## v0.7.0
- It is now open-source.

[#1]: https://github.com/KennethTrecy/chearmyp_lexer/issues/1
