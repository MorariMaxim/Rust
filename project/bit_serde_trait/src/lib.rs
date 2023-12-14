

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use std::vec;

pub use::bit_serde_macro::*;
 
pub trait BitSerdeDeserialization {
    fn deserialize(data: Vec<u8>) -> Self;


}

pub trait BitSerdeSerialization {
    fn serialize(&self) -> std::io::Result<Vec<u8>>;
    fn write_starting_from(&self, destination: &Vec<u8>, next: usize) -> std::io::Result<usize>;

    
}
fn write_n_bits_starting_from(destination: &Vec<u8>, next: usize, data:u8, n: u8)  -> std::io::Result<usize> {


    Ok(1)
}

impl<E: BitSerdeSerialization> BitSerdeSerialization for Vec<E> {

    fn serialize(&self) -> std::io::Result<Vec<u8>> {

        let data = Vec::<u8>::new();
        let next = 0usize;  

        self.write_starting_from(&data, next)?;
        
        Ok(data)
    }

    fn write_starting_from(&self, destination: &Vec<u8>, next: usize) -> std::io::Result<usize> {
        
        let len = self.len();
        
        write_n_bits_starting_from(destination, next, len as u8, 8)?;

        Ok(0usize)
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
