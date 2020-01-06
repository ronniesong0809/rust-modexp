use std::env;
use std::str::FromStr;
use std::io::Write;
use std::io;
use std::convert::TryInto;

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

fn modexp(a: u64, b: u64, c: u64) -> u64 {
    let result = a.pow(b.try_into().unwrap()) % c;
    result
}