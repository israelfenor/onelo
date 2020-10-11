# How to contribute

Onelo is in a very early stage, thanks for chipping in!


## Setup

Onelo requires a standard Rust setup with Rust version equal or greater than 1.46.

### GNU/Linux

Make sure you also have the development packages of openssl installed.
For example, `libssl-dev` on Ubuntu or `openssl-devel` on Fedora.

## Testing

Make sure your contribution has the right test coverage to ensure we
have a healthy level of confidence things run the way they were intended to.


## Documentation

Make sure you explain the _intention_ of a piece of code (e.g. function), the
“why”. The codebase itself should explain the “how”, so use succinct names
that conveys its intention well.


## Commits

We adhere to the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/).

In particular we use:

* `feat`, for features.
* `fix`, for bug fixes.
* `chore`, for supporting changes such as cleaning, reordering, or tweaking
  the working environment.

DO NOT USE: `docs`, `style`, `refactor`, `perf` or `test`. We believe a
feature is composed of documentation and tests.

Finally, use the imperative form to write your commit messages.
