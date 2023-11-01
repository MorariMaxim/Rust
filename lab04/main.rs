use std::{io, fs, string, result};
use std::fs::File;
use std::io::Write;
use std::time::Instant;
  

fn main() { 

    

}

fn p1(path : &str){
    let s = fs::read_to_string(path).unwrap();
    
    let mut maxcharlen = 0;
    let mut maxchar : String = String::from("");    
    let mut maxbytelen = 0;
    let mut maxbyte : String  = String::from("");
    let mut clen = 0usize;
    for line in s.lines() {

        //println!("{line}");
        clen = line.len();

        if clen > maxbytelen { maxbytelen = clen; maxbyte = String::from(line) ;}

        clen = 0;

        for c in line.chars() {
            clen += 1;
        }

        if clen > maxcharlen {maxchar = String::from(line); maxcharlen = clen;}
        
    }   
    println!("{}\n{}",maxchar,maxbyte);
}

enum MyError {
    NotAscii,
    WrongPath,
    Unforeseen
}
fn p2(path : &str) -> Result<(), MyError>{

    let r = fs::read_to_string(path);
    let  s : String;
    if r.is_ok() {s=r.unwrap() }
    else { return  Err(MyError::WrongPath); }

    let mut res = String::from("");
    let mut t : u8;
    for c in s.chars() {
        
        if !c.is_ascii() { return Err(MyError::NotAscii);}

        if c>='a' && c<='z' {
            t = 97 + ((c as u8 - 97 + 13) % 26);
        }
        else if c>='A' && c<='Z' {
            t = 65 + ((c as u8 - 65 + 13) % 26);
        }
        else { t = c as u8}

        res.push(t as char);
    }

    println!("{res}");
    Ok(()) 
}  

fn p3(path : &str) -> Result<(), MyError>{

    let r = fs::read_to_string(path);
    let  s : String;
    if r.is_ok() {s=r.unwrap() }
    else { return  Err(MyError::WrongPath); }

    let mut res = String::from("");

    for comp in s.split(' ') {

        if comp.eq("pt") || comp.eq("ptr") {
            res.push_str("pentru "); continue;
        }
        else if comp.eq("dl") {
            res.push_str("domnul "); continue;
        }
        else if comp.eq("dna") {
            res.push_str("doamna "); continue;
        }
        else {res.push_str(comp); res.push(' ')}
    }
    println!("{res}");
    Ok(())
}


fn p4 () -> Result<(), MyError> {

    let r = fs::read_to_string("C:\\Windows\\System32\\drivers\\etc\\hosts"); 

    let  s : String;
    if r.is_ok() {s=r.unwrap() }
    else { return  Err(MyError::WrongPath); }

    let mut res = String::from("");

    for line in s.lines() {

        if let Some(c) = line.chars().next() {
            if c == '#' {
                continue;
            }
        } 

        let mut t = line.split_whitespace();
        
        let s1 = t.next();
        if s1.is_none() {continue;};        
        let s2 = t.next();
        if s2.is_none() {continue;};        
        
        res.push_str(s2.unwrap());
        res.push_str(" => ");
        res.push_str(s1.unwrap());
        res.push('\n');

    }

    println!("{res}");
    Ok(())
}
fn do_stuff() {
    for _ in 0..1_000_000 {
        print!("");
    }
}
