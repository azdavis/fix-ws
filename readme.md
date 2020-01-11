# fix-ws

Fixes whitespace by:

- Removing trailing whitespace.
- Ensuring there is exactly one LF at EOF.
- Optionally converting indentation between tabs and spaces.

## Installation

Install [brew][], and then

```
$ brew install azdavis/formulae/fix-ws
```

## Development

Install [rust][], clone this repo, `cd` inside, and then

```
$ cargo build
$ cargo test
```

[brew]: https://brew.sh
[rust]: https://www.rust-lang.org
