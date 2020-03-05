# unique

![Rust](https://github.com/ronniesong0809/unique/workflows/Rust/badge.svg?branch=master)

Copyright (c) 2020 Ronnie Song

This is a Rust based "unique" library crate that find the unique value from an iterator safisfying a predicate.

## Usage

Check if 4 is unique in the nums.
```rust
use unique::Unique;

let mut nums = vec![];
let num: fn(&&usize) -> bool = |&&n| n == 4;

nums.push(4);
assert_eq!(Some(&4), nums.iter().unique(even)); //true
```

## Run Example

To run the example program, type the command below:

```shell
$ cargo run --example example
```
```
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
     Running `target/debug/examples/example`
None
None
Some(0)
Some(1)
None
None
Some(3)
None
Some("Ronnie")
Some("David")
Some("Ronnie")
None
```

Everything went well! It check and find the unique value from an iterator satisfying a predicate without issues, and successfully print results as expected.

## Test

To test the library crate, type the command below:

```
$ cargo test
```
```
   Compiling unique v0.1.0 (/mnt/c/Users/ronsong/Desktop/Docs/Rust Programming/unique)
    Finished dev [unoptimized + debuginfo] target(s) in 1.71s
     Running target/debug/deps/test-e7e23989fe7c65fd

running 8 tests
test test_cases::empty ... ok
test test_cases::unique_even ... ok
test test_cases::unique_odd ... ok
test test_cases::nonunique_even_or_odd ... ok
test test_cases::unique_number ... ok
test test_cases::nonunique_number ... ok
test test_cases::unique_name ... ok
test test_cases::nonunique_name ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

All tests passed with no issues.

The tests are placed in tests/test.rs file that uses std assert_eq!() to test equality of the actual result and expected result of the unique() trait in that file.

Github Action CI is running to do the automated testing. ![Rust](https://github.com/ronniesong0809/unique/workflows/Rust/badge.svg?branch=master)

## License

This program is licensed under the "MIT License". Please see the file `LICENSE` in the source distribution of this software for license terms.
