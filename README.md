# fix-ws

Fixes whitespace by:

- Removing trailing whitespace.
- Ensuring there is exactly one LF at EOF.
- Removing all CR.
- Optionally converting indentation between tabs and spaces.

## Usage

Fix whitespace in-place:

```
$ fix-ws foo.txt
```

Fix whitespace, while also converting 2 spaces of indentation to a tab:

```
$ fix-ws -t 2 bar.js
```

Fix whitespace, while also converting a tab of indentation to 4 spaces on many files at a time:

```
$ fix-ws -s 4 quz.rs main.rs
```

## Development

Install [rust][], download the repo, enter it, and then:

```
$ cargo build
$ cargo test
```

[rust]: https://rustup.rs
