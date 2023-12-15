pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub use ::bit_serde_macro::*;
extern crate byteorder;
use bitvec::prelude::*;
use bitvec::view::BitView;

//           Main traits

pub trait BitSerdeDeserialization {
    const SIZE_IN_BITS: usize;
    fn deserialize(data: &Vec<u8>) -> Self;
    fn deserialize_from(data: &BitSlice<u8, Lsb0>) -> (&BitSlice<u8, Lsb0>, Self);
}
pub trait BitSerdeSerialization {
    fn serialize(&self) -> std::io::Result<Vec<u8>>;
    fn write_bits_to(&self, destination: &mut BitVec<u8, Lsb0>) -> std::io::Result<()>;
}

//      IMplementation for Vectors
impl<E: BitSerdeSerialization> BitSerdeSerialization for Vec<E> {
    fn serialize(&self) -> std::io::Result<Vec<u8>> {
        let mut destination = bitvec!(u8,Lsb0;);

        self.write_bits_to(&mut destination)?;

        Ok(destination.into())
    }
    fn write_bits_to(&self, destination: &mut BitVec<u8, Lsb0>) -> std::io::Result<()> {
        let len = self.len();

        destination.extend(len.view_bits::<Lsb0>());

        for el in self.iter() {
            el.write_bits_to(destination)?;
        }

        Ok(())
    }
}

impl<E: BitSerdeDeserialization> BitSerdeDeserialization for Vec<E> {
    const SIZE_IN_BITS: usize = <E>::SIZE_IN_BITS;

    fn deserialize(data: &Vec<u8>) -> Self {
        let bs = data.view_bits::<Lsb0>();

        BitSerdeDeserialization::deserialize_from(bs).1
    }
    fn deserialize_from(mut data: &BitSlice<u8, Lsb0>) -> (&bitvec::slice::BitSlice<u8>, Vec<E>) {
        const SIZE: usize = std::mem::size_of::<usize>();

        let bytes = data[0..SIZE * 8].to_bitvec().into_vec();

        if bytes.len() != SIZE {
            panic!("uneuqal sizes");
        }

        let mut x: [u8; SIZE] = [0; SIZE];

        let mut it = 0u8;
        for i in bytes {
            x[it as usize] = i;
            it += 1;
        }

        let len: usize = <usize>::from_le_bytes(x);
        println!("size of vec = {len}");

        let mut vec = Vec::<E>::with_capacity(len);

        data = &data[SIZE * 8..];

        for _ in 0..len {
            let result = BitSerdeDeserialization::deserialize_from(data);

            vec.push(result.1);
            data = result.0;
        }

        (data, vec)
    }
}

// Implementation for number types
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

                    let bytes = self.to_le_bytes();

                    destination.extend(bytes.iter());

                    Ok(())
                }
            }

            impl BitSerdeDeserialization for $t {
                const SIZE_IN_BITS:usize = mem::size_of::<$t>();

                fn deserialize(data: &Vec<u8>) -> Self {

                    let bs = data.view_bits::<Lsb0>();

                    BitSerdeDeserialization::deserialize_from(bs).1
                }

                fn deserialize_from(mut data: &BitSlice<u8,Lsb0>) -> (&BitSlice<u8, Lsb0>,Self) {

                    const SIZE : usize = mem::size_of::<$t>();

                    let bytes = data[0..SIZE*8].to_bitvec().into_vec();

                    let mut x: [u8;mem::size_of::<$t>()] = [0;mem::size_of::<$t>()];

                    let mut it = 0u8;
                    for i in bytes {
                        x[it as usize] = i;
                        it+=1;
                    }
                    data = &data[SIZE*8..];

                    (data,<$t>::from_le_bytes(x))
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
