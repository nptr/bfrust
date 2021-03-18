# bfrust

A simple brainfuck interpreter written in Rust for learning purposes, because a "Hello World!" is not a realistic start. The interpreter provides 64000 cells, each 8-bit wide. There are no optimisations except that the jump destinations of ']' and '[' are resolved in a preprocessing step and stored in a plain hash map.

Most programs run fine. Find some examples at:<br>
https://github.com/rdebath/Brainfuck/tree/master/testing
https://github.com/saulpw/brainfuck/tree/master/tests

## Usage
In a terminal, call
`bfr <filepath>`

## Building
Provided you have the Rust toolchain installed, just call
`cargo build`