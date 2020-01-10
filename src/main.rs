// Ronnie Song

use std::env;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(arg);
    }

    if numbers.len() != 3 {
        error();
    }

    for i in 0..3 {
        if numbers[2] == "0" {
            error();
        }

        let x = numbers[i].parse::<u32>();
        match x {
            Ok(_ok) => (),
            Err(e) => {
                eprint!("\n{} ", e);
                error()
            }
        }
    }

    let a = numbers[0].parse().expect("err parsing");
    let b = numbers[1].parse().expect("err parsing");
    let c = numbers[2].parse().expect("err parsing");

    let result = modexp(a, b, c);

    println!("\nThe modexp of {} ** {} (mod {}) is {}", a, b, c, result);
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
    eprintln!("\nmodexp: usage: cargo run [x] [y] [m]");
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
