# Chearmyp Lexer
A lexer for Chearmyp language.

This library represents the source as a queue of tokens.

## Origin
It was in a repository with the [parser library]. Yet it has been forked as some possible use cases
may not need a parser.

Some parts of the repository was based from [`filled_bare_metal`] branch of [Feo Template].

## Tokens
Most functions in the library uses [abstract tokens and token queues] so it can be used with different
implementations of [Chearmyp token and token queues].

### Author
Coded by Kenneth Trecy Tobias.

[`filled_bare_metal`]: https://github.com/KennethTrecy/feo_template/tree/filled_bare_metal
[Feo Template]: https://github.com/KennethTrecy/feo_template
[parser library]: https://github.com/KennethTrecy/chearmyp_parser
[abstract tokens and token queues]: https://github.com/KennethTrecy/abstract_chearmyp_token
[Chearmyp token and token queues]: https://github.com/KennethTrecy/chearmyp_token
