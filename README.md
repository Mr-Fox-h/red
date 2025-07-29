# red 🛋️

A simple `cat` clone written in Rust. `red` (short for "read") is a lightweight command-line tool to display file contents with optional line numbers and non-printable character visualization.

## Usage
If you want to read a file, just type `red <FILE NAME>`.

```bash
$ red -h
Usage: red [OPTIONS] [FILE]

Arguments:
  [FILE]  

Options:
  -n, --no-num           Hide line number
  -a, --show-all         show non-printable characters
  -o, --output <OUTPUT>  copy all content into a file
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

## Installation

Clone the repository or install that with `cargo`.
### From source (requires Rust toolchain)
```bash
$ cargo install --git https://github.com/Mr-Fox-h/red
```
