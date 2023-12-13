use thiserror::Error;

fn main() {
    repeatcall();
}

//P1
fn is_prime(x: u32) -> bool {
    if x <= 1 {
        return false;
    }
    if x <= 3 {
        return true;
    }
    for i in 2..(x as f64).sqrt() as u32 + 1 {
        if x % i == 0 {
            return false;
        }
    }
    true
}

fn next_prime(x: u16) -> Option<u16> {
    let mut n = x as u32 + 1;
    loop {
        if is_prime(n) {
            if n > 65535 {
                return None;
            } else {
                return Some(n as u16);
            }
        } else {
            n += 1;
        }
    }
}

fn repeatcall() {
    let mut x = 10u16;
    let mut t = x;

    while let r = next_prime(x) {
        x = r.unwrap();
        println!("Next prime after {t} is {x}");
        t = x;
    }
    println!("No more primes lower than 65535");
}

//P2

fn add_panic(x: u32, y: u32) -> u32 {
    let s1: u64 = x as u64 + y as u64;
    let s2 = s1 as u32;

    if s1 != s2 as u64 {
        panic!("Sum overflowing u32 limits")
    }

    return s2;
}
fn multiply_panic(x: u32, y: u32) -> u32 {
    let p1: u64 = x as u64 * y as u64;
    let p2 = p1 as u32;

    if p1 != p2 as u64 {
        panic!("Sum overflowing u32 limits")
    }

    return p2;
}

//P3
#[derive(Error, Debug)]
enum Myerror1 {
    #[error("{0} overflows the intended limits")]
    SumOverflow(u64),
    #[error("{0} overflows the intended limits")]
    ProductOverflow(u64),
}
fn add_r(x: u32, y: u32) -> Result<u32, Myerror1> {
    let s1: u64 = x as u64 + y as u64;
    let s2 = s1 as u32;

    if s1 != s2 as u64 {
        return Err(Myerror1::SumOverflow(s1));
    }

    Ok(s2)
}
fn multiply_r(x: u32, y: u32) -> Result<u32, Myerror1> {
    let p1: u64 = x as u64 * y as u64;
    let p2 = p1 as u32;

    if p1 != p2 as u64 {
        return Err(Myerror1::ProductOverflow(p1));
    }

    Ok(p2)
}

fn propagating(x: u32, y: u32) {
    let s = add_r(x, y);

    match s {
        Ok(r) => {
            println!("Sum of {x} and {y} is {r}")
        }
        Err(e) => {
            println!("An error occurred: {:?}", e)
        }
    }

    let p = multiply_r(x, y);

    match p {
        Ok(r) => {
            println!("Product of {x} and {y} is {r}")
        }
        Err(e) => {
            println!("An error occurred: {:?}", e)
        }
    }
}

//P4
#[derive(Error, Debug)]
enum CharErrorType {
    #[error("value \'{0}\' does not represent an ascii character")]
    NotAscii(u32),
    #[error("value \'{0}\' does not represent a digit character")]
    NotDigit(u32),
    #[error("value \'{0}\' does not represent a hex character")]
    NotHex(u32),
    #[error("value \'{0}\' does not represent a letter character")]
    NotLetter(u32),
    #[error("value \'{0}\' does not represent a printable character")]
    NotPrintable(u32),
}
fn to_uppercase(c: char) -> Result<char, CharErrorType> {
    let a = c as u8;

    if !(c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z') {
        return Err(CharErrorType::NotLetter(a as u32));
    }
    if c >= 'a' && c <= 'z' {
        return Ok((a + 32) as char);
    }

    Ok(c)
}

fn to_lowercase(c: char) -> Result<char, CharErrorType> {
    let a = c as u8;

    if !(c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z') {
        return Err(CharErrorType::NotLetter(a as u32));
    }
    if c >= 'A' && c <= 'Z' {
        return Ok((a - 32) as char);
    }

    Ok(c)
}

fn char_to_number(c: char) -> Result<u8, CharErrorType> {
    let a = c as u8;

    if !(c >= '0' && c <= '9') {
        return Err(CharErrorType::NotDigit(a as u32));
    }
    Ok(a - 48)
}
fn char_to_number_hex(c: char) -> Result<u8, CharErrorType> {
    let mut a = c as u8;

    if !(c >= '0' && c <= '9' || c >= 'A' && c <= 'F') {
        return Err(CharErrorType::NotHex(a as u32));
    }
    if c >= '0' && c <= '9' {
        return Ok(a - 48);
    } else {
        a -= 'A' as u8;
        a += 10;
    }
    Ok(a)
}
fn print_error(e: CharErrorType) {
    println!("Error type : {:?}", e);
}
fn print_char(c: char) -> Result<(), CharErrorType> {
    if !(c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c >= '0' && c <= '9') {
        return Err(CharErrorType::NotPrintable(c as u32));
    }
    let s: &str = "{}[]-_=+()*&^%$#@!`~<>,./?;:'\"\\|";

    if s.find(c).is_none() {
        return Err(CharErrorType::NotPrintable(c as u32));
    }
    println!("character to be printed : {c}");
    Ok(())
}

//P5
use std::io;
fn partial_calculator() {
    println!("Enter two natural numbers, then either '*', or '+', and we shall see whether the operation overflows using u32 representation");

    let mut num1: i32 = -1;
    let mut num2: i32 = -1;

    let c = 0;
    loop {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => {
                if num1 == -1 {
                    num1 = i as i32;
                } else if num2 == -1 {
                    num2 = i as i32;
                    break;
                }
            }
            Err(..) => println!("this was not a natural number: {}", trimmed),
        };
    }
    let mut op: char;
    loop {
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("failed to read from stdin");

        let trimmed = input_text.trim();
        if trimmed.len() == 1 {
            let o = trimmed.chars().next().unwrap(); // Get the first character from the trimmed string
            if o == '+' || o == '*' {
                op = o;
                break;
            } else {
                println!("This was neither '*' nor '+': {}", o);
            }
        } else {
            println!("Input should be a single character: {}", trimmed);
        }
    }

    if op == '+' {
        let r = add_r(num1 as u32, num2 as u32);
        match r {
            Ok(s) => {
                println!("Result is {s}")
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
    } else {
        let r = multiply_r(num1 as u32, num2 as u32);
        match r {
            Ok(s) => {
                println!("Result is {s}")
            }
            Err(e) => {
                println!("{:?}", e)
            }
        }
    }
}