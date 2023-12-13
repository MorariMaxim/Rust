/// Print information about the crate at startup.
#[cfg(target_os = "windows")]
fn print_crate_info() {
    println!(
        "encoder, version {}, built for windows",
        env!("CARGO_PKG_VERSION")
    );
}
#[cfg(target_os = "linux")]
fn print_crate_info() {
    println!(
        "encoder, version {}, built for linux",
        env!("CARGO_PKG_VERSION")
    );
}
#[cfg(target_os = "macos")]
fn print_crate_info() {
    println!(
        "encoder, version {}, built for macos",
        env!("CARGO_PKG_VERSION")
    );
}
 
use clap::Parser;
#[derive(Parser)]
#[command(version, about = "encoder args")]
struct EncoderArgs {
    /// input file
    #[arg(short, long, default_value = "")]
    input: String,

    /// output file
    #[arg(short, long, default_value = "")]
    output: String,
}

use std::fs::File;
use std::io::{self, Read, Write}; 


fn read_encode_output<R: Read, W: Write>(input: R, mut output:  W) -> io::Result<()>     {
    
    let mut to_encode = Vec::<u8>::new();
    input.take(usize::MAX as u64).read_to_end(&mut to_encode)?;


    let encoded = base64::encode(&to_encode); 
    output.write_all(encoded.as_bytes())?;
    
    Ok(())
}


fn main() -> io::Result<()>{
     
    print_crate_info();

    let mut input: Box<dyn Read> = Box::new(std::io::stdin());

    let mut output: Box<dyn Write> = Box::new(std::io::stdout());

    let args = EncoderArgs::parse();

    if args.input != "" {
        //println!("input file = {}", args.input);
        let file = File::open(args.input).expect("Failed to open input file");
        input = Box::new(file)
    }
    if args.output != "" {
        //println!("output file = {}", args.output);
        let file = File::create(args.output)?;
        output = Box::new(file)
    }

    read_encode_output(input, output)?;

    Ok(())
}
