# grrs-lazy

A simple search utility, based on the `rust` `cli-wg` tutorial:
https://rust-lang-nursery.github.io/cli-wg/tutorial/

It has a few differences from the final `grrs` built in the tutorial:

1. Separated out `find_matches` in `src/lib.rs`.
2. Using `std::io::BufReader` instead of `std::fs::read_to_string`,
   so that we avoid loading all the content from a file in memory.
3. Dealing with iterators in `find_matches`, so that the processing is
   done in a lazy stream.
