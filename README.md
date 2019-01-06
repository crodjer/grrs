# grrs-lazy

[![Crate](https://img.shields.io/crates/v/grrs-lazy.svg)](https://crates.io/crates/grrs-lazy)
[![Build Status](https://travis-ci.com/crodjer/grrs-lazy.svg?branch=master)](https://travis-ci.com/crodjer/grrs-lazy)

A simple search utility, based on the `rust` `cli-wg` tutorial:
https://rust-lang-nursery.github.io/cli-wg/tutorial/

It has a few differences from the final `grrs` built in the tutorial:

1. Separated out `find_matches` in `src/lib.rs`.
2. Using `std::io::BufReader` instead of `std::fs::read_to_string`,
   so that we avoid loading all the content from a file in memory.
3. Dealing with iterators in `find_matches`, so that the processing is
   done in a lazy stream.

## Installation
To install, run:
```
cargo install grrs-lazy
```

You can then search for lines in a text file:
```
$ grrs-lazy Sherlock path/to/the-adventures-of-sherlock-holmes.txt | wc -c
6089
```
