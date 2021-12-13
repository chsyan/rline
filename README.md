# rline

rline is a utility to randomize lines of a file.

## Building

A [Rust installation](https://www.rust-lang.org/) is required in order to compile the program.

To build rline:

```
$ git clone https://github.com/chsyan/rline
$ cd rline
$ cargo build --release
```

The built binary can be found at `./target/release/rline`.

## Usage

The following will randomize the lines of `input.txt` and save it to `output.txt`

```
$ rline input.txt > output.txt
```
