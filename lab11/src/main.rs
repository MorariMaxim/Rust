use std::{fs,io::{Write, Result}};

struct MyWriter {
    object:  Box<dyn Write>,
}

impl MyWriter {
    fn new<T: Write + 'static >(init_val: T) -> Self {
        MyWriter {
            object: Box::new(init_val),
        }
    } 
}

impl Write for MyWriter {
       fn write(&mut self, buf: &[u8]) -> Result<usize> {            
            let mut buf2 = Vec::<u8>::with_capacity(buf.len()*2);
            for byte in buf.iter(){
                buf2.push(byte.to_owned());
                buf2.push(byte.to_owned());
            }
            self.object.write(&buf2)?;
            Ok(buf.len())
       } 
       fn flush(&mut self) -> Result<()> {
           self.object.flush()
       }       
}
 

fn main() -> std::io::Result<()> {
    let mut writer = MyWriter::new(fs::File::create("a.txt")?);
    writer.write_all(b"abc")?; 

    Ok(())
}
