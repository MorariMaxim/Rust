pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub use ::bit_serde_macro::*;
extern crate byteorder;
use bitvec::prelude::*;
use bitvec::view::BitView;
use byteorder::{BigEndian, ByteOrder, LittleEndian};

pub trait BitSerdeDeserialization {
    fn deserialize(data: &Vec<u8>) -> Self;
    fn deserialize_from(data: &BitVec<u8, Lsb0>) -> Self;
}
pub trait BitSerdeSerialization {
    fn serialize(&self) -> std::io::Result<Vec<u8>>;
    fn write_bits_to(&self, destination: &mut BitVec<u8, Lsb0>) -> std::io::Result<()>;
}

impl<E: BitSerdeSerialization> BitSerdeSerialization for Vec<E> {
    fn serialize(&self) -> std::io::Result<Vec<u8>> {
        let data = Vec::<u8>::new();
        let next = 0usize;

        //self.write_starting_from(&data, next)?;

        Ok(data)
    }
    fn write_bits_to(&self, destination: &mut BitVec<u8, Lsb0>) -> std::io::Result<()> {
        Ok(())
    }
}
use std::mem;
macro_rules! implement_traits_for_unsigned_types {
    ($($t:ty),*) => {
        $(
            impl BitSerdeSerialization for $t {
                fn serialize(&self) -> std::io::Result<Vec<u8>> {

                    let mut destination = bitvec![u8, Lsb0;];

                    BitSerdeSerialization::write_bits_to(self,&mut destination)?;

                    Ok(destination.into_vec())
                }
                fn write_bits_to(&self, destination: &mut BitVec<u8,Lsb0>) -> std::io::Result<()> {

                    let bytes = self.to_ne_bytes();

                    destination.extend(bytes.iter()); 

                    Ok(())
                }
            }

            impl BitSerdeDeserialization for $t {

                fn deserialize(data: &Vec<u8>) -> Self {

                    const SIZE : usize = mem::size_of::<$t>();

                    if data.len() != SIZE {
                        panic!("unequal sizes");
                    }

                    let mut x: [u8;SIZE] = [0;SIZE];

                    let mut it = 0u8;

                    for i in data {
                        x[it as usize] = *i;
                        it+=1;
                    }

                    <$t>::from_ne_bytes(x)
                }
                fn deserialize_from(data: &BitVec<u8,Lsb0>) -> Self {

                    const SIZE : usize = mem::size_of::<$t>();

                    let bytes = data[0..SIZE*8].to_bitvec().into_vec();

                    let mut x: [u8;mem::size_of::<$t>()] = [0;mem::size_of::<$t>()];

                    let mut it = 0u8;
                    for i in bytes {
                        x[it as usize] = i;
                        it+=1;
                    }

                    <$t>::from_ne_bytes(x)
                }
            }
        )*
    };
}
implement_traits_for_unsigned_types!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
