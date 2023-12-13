use std::{io::repeat, string};

fn main() {

    //p3(); 
    //return;
    let mut s = String::from("abc");
 

    let mut i = 0;
    while i < 26 {
        let c = (i as u8 + 'a' as u8) as char;
        //add_chars_n(&mut s, c, 26 - i);
        s = add_chars_n(s, c, 26 - i);
        i += 1;
    }

    print!("{}", s);
}

//P1
fn add_chars_n(mut s: String ,c:char,n:i32 )->String{
    
    for _i in 0..n {
        s.push(c);
    }
    s
}    

//P2
fn add_chars_n_p2(s:&mut String,c:char,n:i32 ){
    
    for _i in 0..n {
        (*s).push(c);
    } 
} 


//P3

fn add_space(s:&mut String, n: i32) {
    for _i in 0..n {
        s.push(' ');
    }
}

fn add_str(s1:&mut String,s2: &str ) {
    s1.push_str(s2);
}

fn add_integer(s:&mut String, mut num : i32){

    let mut d:u8 = 0; 
    let mut temp : String = String::from("");    
    let mut neg :u8 = 0;

    if num<0 {neg=1;num*=-1;}
    loop{

        d = (num %10) as u8;
        num = num/10;
        temp.push((48+ d) as char );
        
        if num==0 {
            break;
        }
    }
    if neg==1 {s.push('-')};
    let mut it = 0;
    for c in temp.chars().rev(){        
        if it%3 == 0 && it!=0 {s.push('_')};
        s.push(c);        
        it+=1;
    }
}

fn add_float(s: &mut String, mut num:f32){

    let mut neg = 0;
    if num<0.0 {
        num*=-1f32;
        neg = 1;
    }    

    let p1  = num.floor() as i32;
    let mut p2_ = num - num.floor();    
    
    let mut temp =  String::from("");
    let mut t:i32;
    loop {   
        t = p2_.floor() as i32;
        p2_*=10f32;
        if (p2_-p2_.floor()) == 0f32 {break;}
        if t==p2_.floor()as i32 {temp.push('0');};
    }

    let p2 = p2_ as i32;

    if neg==1 {s.push('-')};

    if p1 !=0 && p2!=0{
        add_integer(s, p1);
        s.push('.');
        s.push_str(&temp);
        add_integer(s, p2);
    }
    else if p1 !=0 && p2==0 {
        add_integer(s, p1); 
    }
    else if p1 ==0 && p2!=0{        
        s.push_str("0.");
        s.push_str(&temp);
        add_integer(s, p2);
    }
    else{        
        s.push('0');        
    }
}

fn p3(){

    let mut s = String::from("");
    add_space(&mut s, 40);
    add_str(&mut s, "I 💚\n");
    add_space(&mut s, 40);
    add_str(&mut s, "Rust.\n\n");
    add_space(&mut s, 4);
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "crate");
    add_space(&mut s, 6);
    add_integer(&mut s, 306437968);
    add_space(&mut s,11);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "latest");
    add_space(&mut s, 9);
    add_str(&mut s, "is");
    add_str(&mut s, "\n");

    add_space(&mut s, 9);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 8);
    add_str(&mut s, "has");
    add_space(&mut s, 13);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 9);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038);
    add_str(&mut s, ".");

    println!("{s}");
}