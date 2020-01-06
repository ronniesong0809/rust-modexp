use std::env;
use std::str::FromStr;
use std::io::Write;
use std::io;

fn main() {
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str( & arg).expect("err parsing"));
    }

    if numbers.len() != 3 {
        writeln!(io::stderr(), "Usage: cargo run [number] [number] ...").unwrap();
        std::process::exit(1);
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

    z = z * z % m;

    if y % 2 != 0 {
        z = z * x % m;
    }
    z
}