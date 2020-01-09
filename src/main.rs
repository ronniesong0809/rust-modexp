// Ronnie Song

use std::env;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        // take only non-negative x and y and positive m, and all should be less than 2**32
        let num = u32::from_str(&arg).expect("Error Parsing");
        numbers.push(num);
    }

    if numbers.len() != 3 {
        error();
    }

    // gives the maximum possible 32-bit number as a u64
    let (a, b, c) = (
        u64::from(numbers[0]),
        u64::from(numbers[1]),
        u64::from(numbers[2]),
    );

    let result = modexp(a, b, c);

    println!("The modexp of {} ** {} (mod {}) is {}", a, b, c, result);
}

fn modexp(x: u64, y: u64, m: u64) -> u64 {
    if x == 0 {
        return 0;
    }

    if y == 0 {
        return 1;
    }

    let mut z = modexp(x, y / 2, m);

    z = modulo(z * z, m);

    if y % 2 != 0 {
        z = modulo(z * x, m);
    }
    z
}

fn modulo(x: u64, y: u64) -> u64 {
    let rem = x % y;
    // println!("x is {}, y is {}, result is {}", x, y, rem);
    rem
}

fn error() -> ! {
    eprintln!("modexp: usage: cargo run [x] [y] [m]");
    std::process::exit(1);
}

#[cfg(test)]
fn test_modexp(a: u64, b: u64, c: u64, d: u64) {
    assert_eq!(modexp(a, b, c), d);
}

#[test]
fn test_case1() {
    test_modexp(2, 20, 17, 16);
}

#[test]
fn test_case2() {
    test_modexp(13, 5, 11, 10);
}

#[test]
fn test_case3() {
    test_modexp(3, 2, 5, 4);
}

#[test]
fn test_case4() {
    test_modexp(100, 2, 7777, 2223);
}
