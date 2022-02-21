# Chearmyp Lexer
A lexer for Chearmyp language.

This library represents the source as a queue of tokens.

## Origin
It was in a repository with the parser library. Yet it has been forked as some possible use cases
may not need as parser.

The repository was based from [`filled_bare_metal`] branch of [Feo Template].

## Tokens
The tokens only contain the most important data of a Chearmyp node. For example, a simplex node can
have no attachers but it cannot exist without the concept name. The list below gives specific
details about each tokens.
- *Scope level*. This token represents the indention of the subsequent token(s).
- *Simplex*. As stated above, a simplex token must contain the concept name only because it is the
  most important data.
- *Complex*. Same with the simplex, the concept name of a complex is the most important because a
  complex node can have no attachers or contents.
- *Attacher*. The label and content of an attacher are both important to add information to the
  simplex or complex they are attaching to.
- *Comment*. Since there are two types of comment nodes, they also have distinction as tokens.
  - *Line comment*. All of its content are important.
  - *Block comment*. The block that the block comment node contains is important.
- *Othertongue*. Othertongue tokens are almost the same as the comment tokens. Their only difference
  is that they are intended to represent a foreign concept.
  - *Line othertongue*
  - *Block othertongue*

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
