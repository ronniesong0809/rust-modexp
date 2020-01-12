// Ronnie Song

use std::env;

fn main() {
    let mut numbers = Vec::new();

    // handle environment vars from command line
    for arg in env::args().skip(1) {
        let check = arg.parse::<u32>().unwrap_or_else(|_| error());
        numbers.push(check);
    }

    // accept 3 args from command line while the last cant be zero
    if numbers.len() == 3 && numbers[2] != 0 {
        let (a, b, c) = (
            u64::from(numbers[0]),
            u64::from(numbers[1]),
            u64::from(numbers[2]),
        );
        let result = modexp(a, b, c);

        println!("The modexp of {} ** {} (mod {}) is {}\n", a, b, c, result);
    } else {
        error();
    }
}

// modular exponentiation
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

// modulo
fn modulo(x: u64, y: u64) -> u64 {
    x % y
}

// print error message and exit
fn error() -> ! {
    eprintln!("Error parsing, please try again!\nmodexp: usage: cargo run <x> <y> <m>\n");
    std::process::exit(1);
}

// unit testing
#[cfg(test)]
mod test_cases {
    use super::*;

    #[test]
    fn mod_passed_1() {
        assert_eq!(modulo(100, 77), 23);
    }

    #[test]
    fn mod_passed_2() {
        assert_eq!(modulo(77, 33), 11);
    }

    #[test]
    fn mod_failed_1() {
        assert_ne!(modulo(1312412, 12412), 1); // should be 9152
    }

    #[test]
    fn mod_failed_2() {
        assert_ne!(modulo(122, 11), 0); // should be 0
    }

    #[test]
    fn modexp_passed_1() {
        assert_eq!(modexp(2, 20, 17), 16);
    }

    #[test]
    fn modexp_passed_2() {
        assert_eq!(modexp(13, 5, 11), 10);
    }

    #[test]
    fn modexp_failed_1() {
        assert_ne!(modexp(100, 2, 7777), 2222); // should be 2223
    }

    #[test]
    fn modexp_failed_2() {
        assert_ne!(modexp(10, 2, 25), 1); // should be 0
    }
}
