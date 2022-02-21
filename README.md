# Chearmyp Lexer
A lexer for Chearmyp language.

This library represents the source as a queue of tokens.

## Origin
It was in a repository with the parser library. Yet it has been forked as some possible use cases
may not need a parser.

The repository was based from [`filled_bare_metal`] branch of [Feo Template].

## Tokens
Most functions in the library uses abstract tokens so it can be used with different implementations
of Chearmyp token.

## Token Queue Representation
Consider the following Chearmyp text:
```
hello
   name: ABC
   # DEF
```
The token queue will represent the text as series of tokens. It will have a complex token (`hello`),
scope level token (with a value of 1), an attacher token (with `name` label and `ABC` content), and
a line comment token (with a content of ` DEF`) in that order.

### Author
Coded by Kenneth Trecy Tobias.

[`filled_bare_metal`]: https://github.com/KennethTrecy/feo_template/tree/filled_bare_metal [Feo
Template]: https://github.com/KennethTrecy/feo_template
