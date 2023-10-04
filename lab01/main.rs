fn main() {
    problem3();
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    let im: f32 = n as f32;
    let sq: f32 = im.sqrt().floor() + 1.0;

    for a in 2..sq as i32 {
        if n % a == 0 {
            return false;
        }
    }
    return true;
}

fn problem1() {
    for i in 1..100 {
        print!("Number {} is ", i);
        if !is_prime(i) {
            print!("not ");
        }
        println!("prime");
    }
}

fn gcd(mut n1: i32, mut n2: i32) -> i32 {
    if n1 == 0 {
        return n2;
    };
    if n2 == 0 {
        return n1;
    };
    while n1 != n2 {
        if n1 > n2 {
            n1 -= n2;
        } else {
            n2 -= n1;
        }
    }
    return n1;
}

fn problem2() {
    for i1 in 0..101 {
        for i2 in 0..101 {
            print!("{} and {} are ", i1, i2);
            if gcd(i1, i2) != 1 {
                print!("not ")
            }
            println!("coprime");
        }
    }
}

fn problem3() {
    for i in (3..100).rev() {
        println!("{} bottles of beer on the wall,", i);
        println!("{} bottles of beer", i);
        println!("Take one down, pass it around,");
        println!("{} bottles of beer on the wall.", i - 1);
        println!("");
    }
    println!("2 bottles of beer on the wall,");
    println!("2 bottles of beer",);
    println!("Take one down, pass it around,");
    println!("1 bottle of beer on the wall.");
    println!("");

    println!("1 bottle of beer on the wall,");
    println!("1 bottle of beer",);
    println!("Take one down, pass it around,");
    println!("No bottles of beer on the wall.");
    println!("");
}
