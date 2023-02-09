[![Library Tests](https://img.shields.io/github/actions/workflow/status/KennethTrecy/chearmyp_lexer/library.yml?style=for-the-badge)](https://github.com/KennethTrecy/chearmyp_lexer/actions/workflows/library.yml)
![GitHub lines](https://img.shields.io/github/license/KennethTrecy/chearmyp_lexer?style=for-the-badge)
![GitHub release (latest SemVer)](https://img.shields.io/github/v/release/KennethTrecy/chearmyp_lexer?style=for-the-badge&display_name=tag&sort=semver)
![GitHub closed issues count](https://img.shields.io/github/issues-closed/KennethTrecy/chearmyp_lexer?style=for-the-badge)
![GitHub pull request count](https://img.shields.io/github/issues-pr-closed/KennethTrecy/chearmyp_lexer?style=for-the-badge)
![Commits since latest version](https://img.shields.io/github/commits-since/KennethTrecy/chearmyp_lexer/latest?style=for-the-badge)
![Lines of code](https://img.shields.io/tokei/lines/github/KennethTrecy/chearmyp_lexer?style=for-the-badge)
![GitHub code size in bytes](https://img.shields.io/github/repo-size/KennethTrecy/chearmyp_lexer?style=for-the-badge)

# Chearmyp Lexer
A lexer for Chearmyp language.

This library represents the source as a queue of tokens.

## Installation
Add it to the dependencies:
```
[dependencies.chearmyp_lexer]
git = "https://github.com/KennethTrecy/chearmyp_lexer"
tag = "v1.0.0"
```

You may also activate all the features:
```
[dependencies.chearmyp_lexer]
git = "https://github.com/KennethTrecy/chearmyp_lexer"
tag = "v1.0.0"
features = ["no_std"]
```

## Origin
It was in a repository with the [parser library]. Yet it has been forked as some possible use cases
may not need a parser.

Some parts of the repository was based from [`filled_bare_metal`] branch of [Feo Template].

## Code
Most functions in the library uses [abstract tokens and token queues] so it can be used with different
implementations of [Chearmyp token and token queues].

## Usage

### Initialization
If you want to contribute, this repository should be initialized to adhere in [Conventional Commits specification] for organize
commits and automated generation of change log.

#### Prerequisites
- [Node.js and NPM]
- [pnpm] (optional)

#### Instructions
By running the command below, all your commits will be linted to follow the [Conventional Commits
specification].
```
$ npm install
```

Or if you have installed [pnpm], run the following command:
```
$ pnpm install
```

To generate the change log automatically, run the command below:
```
$ npx changelogen --from=[tag name or branch name or commit itself] --to=master
```

## Notes

### License
The repository is licensed under [MIT].

### Want to contribute?
Read the [contributing guide] for different ways to contribute in the project.

### Author
Chearmyp Lexer was created by Kenneth Trecy Tobias.

[`filled_bare_metal`]: https://github.com/KennethTrecy/feo_template/tree/filled_bare_metal
[Feo Template]: https://github.com/KennethTrecy/feo_template
[MIT]: https://github.com/KennethTrecy/chearmyp_lexer/blob/master/LICENSE
[Node.js and NPM]: https://nodejs.org/en/
[pnpm]: https://pnpm.io/installation
[Conventional Commits specification]: https://www.conventionalcommits.org/en/v1.0.0/
[contributing guide]: ./CONTRIBUTING.md
[parser library]: https://github.com/KennethTrecy/chearmyp_parser
[abstract tokens and token queues]: https://github.com/KennethTrecy/abstract_chearmyp_token
[Chearmyp token and token queues]: https://github.com/KennethTrecy/chearmyp_token
