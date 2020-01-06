use std::env;
use std::str::FromStr;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str( & arg).expect("err parsing"));
    }

    if numbers.len() != 3 {
        error();
    }

    let a = numbers[0];
    let b = numbers[1];
    let c = numbers[2];
    
    let result = modexp(a, b, c);

    println!("The modexp of {:?} is {}", numbers, result);
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
    println!("x is {}, y is {}, result is {}", x, y, rem);
    rem
}

fn error() -> ! {
    eprintln!("modexp: usage: cargo run [x] [y] [m]");
    std::process::exit(1);
}