# bfrust

A simple, slightly optimizing, brainfuck interpreter written in Rust for learning purposes, because a "Hello World!" is not a realistic start. The interpreter provides 64000 cells, each 8-bit wide. It optimizes three things: 
* Consecutive `<` or `>` are merged into a `ptr += operand` instruction.
* Consecutive `+` or `-` are merged into a `cell[ptr] += operand` instruction.
* Jump destinations of `[` and `]` are resolved in a preprocessing step.

It should run most programs you'll find. Here are some good tests and examples:<br>
* https://github.com/rdebath/Brainfuck/tree/master/testing
* https://github.com/saulpw/brainfuck/tree/master/tests

## Usage
In a terminal, call
`bfr <filepath>`

```
$ ./bfr Mandelbrot-tiny.b 
AAAAAAAABBBBBBBBCCCCCCCCCCCCCCCCCCDDDDEFEEDDDCCCCCBBBBBBBBBBBBBBB
AAAAAAABBBBBBCCCCCCCCCCCCCCCCCDDDDDDEEFIKGGGDDDDDCCCCBBBBBBBBBBBB
AAAAAABBBBCCCCCCCCCCCCCCCCCDDDDDDDEEEFGHKPIGFEDDDDDCCCCCBBBBBBBBB
AAAAABBBCCCCCCCCCCCCCCCCCDDDDDDDEEEFGPVT  Q[HEEEEDDDCCCCCCBBBBBBB
AAAABBCCCCCCCCCCCCCCCCDDDDDDDEEFFFGGHK      HGFFEEEDDDCCCCCBBBBBB
AAABBCCCCCCCCCCCCCCCDDDDDEEEFGK MJJ NR    YS L HHGIJFDDCCCCCCBBBB
AAABCCCCCCCCCCCCCDDDEEEEEEFFFHI                    MGEDDCCCCCCBBB
AABCCCCCCCCCCCDDEEEEEEEEFFFGY Q                   MHGEEDCCCCCCCBB
AACCCCCCDDDDDEEFLHGGHMHGGGHIR                      QLHEDDCCCCCCCB
ABCCDDDDDDEEEEFGIKU    RLJJL                        IFEDDCCCCCCCB
ACDDDDDDEEEEEGGHOS        QR                        JFEDDDCCCCCCC
ADDDDDEFFFGGHKOPS                                   GEEDDDCCCCCCC
A                                                PJGFEEDDDCCCCCCC
ADDDDDEFFFGGHKOPS                                   GEEDDDCCCCCCC
ACDDDDDDEEEEEGGHOS        QR                        JFEDDDCCCCCCC
ABCCDDDDDDEEEEFGIKU    RLJJL                        IFEDDCCCCCCCB
AACCCCCCDDDDDEEFLHGGHMHGGGHIR                      QLHEDDCCCCCCCB
AABCCCCCCCCCCCDDEEEEEEEEFFFGY Q                   MHGEEDCCCCCCCBB
AAABCCCCCCCCCCCCCDDDEEEEEEFFFHI                    MGEDDCCCCCCBBB
AAABBCCCCCCCCCCCCCCCDDDDDEEEFGK MJJ NR    YS L HHGIJFDDCCCCCCBBBB
AAAABBCCCCCCCCCCCCCCCCDDDDDDDEEFFFGGHK      HGFFEEEDDDCCCCCBBBBBB
AAAAABBBCCCCCCCCCCCCCCCCCDDDDDDDEEEFGPVT  Q[HEEEEDDDCCCCCCBBBBBBB
AAAAAABBBBCCCCCCCCCCCCCCCCCDDDDDDDEEEFGHKPIGFEDDDDDCCCCCBBBBBBBBB
AAAAAAABBBBBBCCCCCCCCCCCCCCCCCDDDDDDEEFIKGGGDDDDDCCCCBBBBBBBBBBBB

Finished in 2033 ms
```

## Building
Provided you have the Rust toolchain installed, call
`cargo build` from the project root or `rustc main.rs` from the `src` directory.