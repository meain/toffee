# toffee

Send in filename and position of any file in any language, toffee
returns the command needed to run the nearest test with the proper
testing lib. It is a WIP, currently supports python and rust.

## Usage

This is the main interface that toffee exposes, but this is mostly
meant to be used from within your editor. The ideal flow will be use
toffee to get the command that you need to run, then use your editor
to start a shell or something that runs the actual test.

> Examle implementation for Emacs [here](https://github.com/meain/dotfiles/blob/e99f99469f92be5bb33b9cee342ca627a914b99d/emacs/.config/emacs/init.el#L941)

```
Usage: toffee <filename> [<line_no>] [--full]

Get command to run to run specific test in a file

Options:
  --full            run full test suite
  --help            display usage information
```

## Example

```shell
$ src/pickers/rust.rs
cargo test pickers::rust

$ src/pickers/rust.rs 83
cargo test pickers::rust::tests::test_simple_find
```

## Install

```shell
cargo install toffee
```
