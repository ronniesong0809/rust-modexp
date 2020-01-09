# modexp

[![Build Status](https://travis-ci.com/ronniesong0809/rust-modexp.svg?branch=master)](https://travis-ci.com/ronniesong0809/rust-modexp)

Copyright (c) 2020 Ronnie Song

This is a Rust based command-line calculator that computes *modexp()*.

<br>

*modexp()* is calculate the modular exponentiation:
```
modexp(x, y, m) = x ** y (mod m)
```

### Run
To run the calculator, type the command below:
```
$ cargo run 2 20 17
```
```
   Compiling...
    Finished...
     Running...
The modexp of 2 ** 20 (mod 17) is 16
```
The calculator print 16. Everything went well! the calculator accept only non-negative x and y and positive m, and accept only if all inputs are less than 2**32 (u32).

### Test
To test the calculator, type the command below:
```
cargo test
```
```
   Compiling...
    Finished...
     Running...

running 4 tests
test test_case1 ... ok
test test_case2 ... ok
test test_case3 ... ok
test test_case4 ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

All tests passed with no issues.

The tests are placed at the bottom of the src/main.rs file that uses *assert_eq!()* to test equality of the actual result and expected result of the *modexp()* function in that file.

Travis CI is running to do the automated testing. [![Build Status](https://travis-ci.com/ronniesong0809/rust-modexp.svg?branch=master)](https://travis-ci.com/ronniesong0809/rust-modexp)

## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.
