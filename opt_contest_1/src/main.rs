use std::fs::File;
use std::io::{Write, Read};
use std::time::Instant;





fn main() {

    writexgb();

    let start = Instant::now();

    let r = rot13("input.txt");

    println!("{:?}", start.elapsed());
    
    if r.is_err() {print!("{:?}",r)}
    
    
}
#[derive(Debug)]
enum MyError {
    NotAscii,
    Other,    
}  

fn rot13(path : &str) -> Result<(),MyError>  {
    
 

    let mut res = File::open(path);
    let mut file : File;
    if res.is_ok() { file = res.unwrap()}
    else {return Err(MyError::Other)}


    res = File::create("output.txt");
    let mut outputfile : File;
    if res.is_ok() { outputfile = res.unwrap()}
    else {return Err(MyError::Other)}




     
    let chunk_size = 4096*2; 
    
    let mut buffer = vec![0u8; chunk_size];
    let mut t : u8;
    loop { 
        
        let res = file.read(&mut buffer);
        let bytes_read  : usize;
        if res.is_ok() { bytes_read = res.unwrap()}
        else {return Err(MyError::Other)}

        if bytes_read == 0 { 
            break;
        } 

        for i in 0..bytes_read {    
            t = buffer[i];
            if t >>7 !=0 {return Err(MyError::NotAscii)};

            if t>=65 && t<=90  {
                buffer[i] = 65 + ((t - 65 + 13) % 26) as u8;
            }
            else if t>=97 && t<=122 {
                buffer[i] = 97 + ((t - 97 + 13) % 26) as u8;
            }
            else {buffer[i] = t}
            
        } 

        outputfile.write_all(&buffer[..bytes_read]);
    } 

    Ok(())
}


fn writexgb() -> Result<(),MyError>  {
 
    let mut res = File::create("input.txt");
    let mut outputfile : File;
    if res.is_ok() { outputfile = res.unwrap()}
    else {return Err(MyError::Other)}




     
    let chunk_size = 4096; 
    
    let mut buffer = vec![0u8; chunk_size];
    let mut t : u8;
 
    let mut y = 0;
    for i in 0..4096 {

        if i%26 == 0 { y=1-y; }
        if y==0 {
            buffer[i] =  65 + (i % 26) as u8;
        }
        else {
            buffer[i] =  97 + (i % 26) as u8;
        } 
                
    } 

    let len = 1024 * 10; 

    let start = Instant::now();

    for i in  0..len {
        outputfile.write_all(&buffer);
    }

    println!("{:?}", start.elapsed());


    Ok(()) 

}
