# modexp

[![Build Status](https://travis-ci.com/ronniesong0809/rust-modexp.svg?token=ysuqwpSTd1nLYmpB7CY5&branch=master)](https://travis-ci.com/ronniesong0809/rust-modexp)

Copyright (c) 2020 Ronnie Song

This is a Rust based command-line calculator that computes *modexp()*.

<br>

*modexp()* is calculate the modular exponentiation:
```
modexp(x, y, m) = x ** y (mod m)
```

## Run
### Valid Input
To run the calculator, type the command below:
```
$ cargo run 2 20 17
```

```
   Compiling ...
    Finished ...
     Running ...

The modexp of 2 ** 20 (mod 17) is 16
```
The calculator print 16. Everything went well! 


### Invalid Input
To see the error message, type the command below
```
$ cargo run 2 a 0 
```

```
   Compiling ...
    Finished ...
     Running ...

Error parsing, please try again!
modexp: usage: cargo run <x> <y> <m>
```

The calculator accept only non-negative x and y and positive m, and accept only if all inputs are less than 2**32 (u32). 

The only problem with it is that the error handling doest handle negative `x` well, because it seems that `cargo`, according to the documentation, interprets everything that starts with a `"-"` hyphen as a flag.

## Test
To test the calculator, type the command below:
```
cargo test
```

```
   Compiling ...
    Finished ...
     Running ...

running 8 tests
test test_cases::mod_failed_1 ... ok
test test_cases::mod_failed_2 ... ok
test test_cases::mod_passed_1 ... ok
test test_cases::mod_passed_2 ... ok
test test_cases::modexp_failed_1 ... ok
test test_cases::modexp_failed_2 ... ok
test test_cases::modexp_passed_1 ... ok
test test_cases::modexp_passed_2 ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

All tests passed with no issues.

The tests are placed at the bottom of the src/main.rs file that uses *assert_eq!()* and *assert_ne!()* to test equality of the actual result and expected result of the *modulo()* and *modexp()* functions in that file.

Travis CI is running to do the automated testing. [![Build Status](https://travis-ci.com/ronniesong0809/rust-modexp.svg?token=ysuqwpSTd1nLYmpB7CY5&branch=master)](https://travis-ci.com/ronniesong0809/rust-modexp)

## License

This program is licensed under the "MIT License".  Please
see the file `LICENSE` in the source distribution of this
software for license terms.
