# fix-ws

Fixes whitespace by:

- Removing trailing whitespace.
- Ensuring there is exactly one LF at EOF.
- Removing all CR.
- Optionally converting indentation between tabs and spaces.

## Installation

Install [brew][], and then

```
$ brew install azdavis/formulae/fix-ws
```

## Usage

Fix whitespace in-place:

```
$ fix-ws foo.txt
```

Fix whitespace, while also converting 2 spaces of indentation to a tab:

```
$ fix-ws -t 2 bar.js
```

Fix whitespace, while also converting a tab of indentation to 4 spaces on many
files at a time:

```
$ fix-ws -s 4 quz.rs main.rs
```

## Development

Install [rust][], clone this repo, `cd` inside, and then

```
$ cargo build
$ cargo test
```

[brew]: https://brew.sh
[rust]: https://www.rust-lang.org
