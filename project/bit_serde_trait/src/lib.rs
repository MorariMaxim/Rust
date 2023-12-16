use bitvec::prelude::*;

//           Main traits

pub trait BitSerdeDeserialization
where
    Self: Sized,
{
    fn deserialize(data: &Vec<u8>) -> Self {
        let bs = data.view_bits::<Lsb0>();

        BitSerdeDeserialization::deserialize_from(bs).1
    }

    fn deserialize_from(data: &BitSlice<u8, Lsb0>) -> (&BitSlice<u8, Lsb0>, Self);
}
pub trait BitSerdeSerialization {
    fn serialize(&self) -> std::io::Result<Vec<u8>> {
        let mut destination = bitvec!(u8,Lsb0;);
        destination.force_align();

        self.write_bits_to(&mut destination)?;
        destination.force_align();

        Ok(destination.into())
    }
    fn write_bits_to(&self, destination: &mut BitVec<u8, Lsb0>) -> std::io::Result<()>;
}

//      IMplementation for Vectors
impl<E: BitSerdeSerialization> BitSerdeSerialization for Vec<E> {
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
    fn deserialize_from(mut data: &BitSlice<u8, Lsb0>) -> (&bitvec::slice::BitSlice<u8>, Vec<E>) {
        const SIZE: usize = std::mem::size_of::<usize>();

        let mut bytes = data[0..SIZE * 8].to_bitvec();
        bytes.force_align();
        let bytes = bytes.into_vec();

        if bytes.len() != SIZE {
            panic!("uneuqal sizes, bytes.len = {}", bytes.len());
        }

        let mut x: [u8; SIZE] = [0; SIZE];

        let mut it = 0u8;
        for i in bytes {
            x[it as usize] = i;
            it += 1;
        }

        let len: usize = <usize>::from_le_bytes(x);

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
                fn write_bits_to(&self, destination: &mut BitVec<u8,Lsb0>) -> std::io::Result<()> {

                    let bytes = self.to_le_bytes();

                    destination.extend(bytes.iter());

                    Ok(())
                }
            }

            impl BitSerdeDeserialization for $t {

                fn deserialize_from(mut data: &BitSlice<u8,Lsb0>) -> (&BitSlice<u8, Lsb0>,Self) {

                    const SIZE : usize = mem::size_of::<$t>();

                    let mut bytes = data[0..SIZE*8].to_bitvec();
                    bytes.force_align();
                    let bytes = bytes.into_vec();

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

//      Implementation for String

impl BitSerdeSerialization for String {
    fn write_bits_to(&self, destination: &mut BitVec<u8, Lsb0>) -> std::io::Result<()> {
        let len = self.len();

        destination.extend(len.view_bits::<Lsb0>());

        for el in self.as_bytes() {
            el.write_bits_to(destination)?;
        }
        Ok(())
    }
}

impl BitSerdeDeserialization for String {
    fn deserialize_from(mut data: &BitSlice<u8, Lsb0>) -> (&BitSlice<u8, Lsb0>, Self) {
        let vec: (&BitSlice<u8, Lsb0>, Vec<u8>) = BitSerdeDeserialization::deserialize_from(data);

        data = vec.0;

        let str = String::from_utf8(vec.1).unwrap();

        (data, str)
    }
}

//      Implementation for Bool
impl BitSerdeSerialization for bool {
    fn write_bits_to(&self, destination: &mut BitVec<u8, Lsb0>) -> std::io::Result<()> {
        destination.push(*self);
        Ok(())
    }
}

impl BitSerdeDeserialization for bool {
    fn deserialize_from(mut data: &BitSlice<u8, Lsb0>) -> (&BitSlice<u8, Lsb0>, Self) {
        let val: bool = data[0];

        data = &data[1..];

        (data, val)
    }
}

//      Implementation for char
impl BitSerdeSerialization for char {
    fn write_bits_to(&self, destination: &mut BitVec<u8, Lsb0>) -> std::io::Result<()> {
        let len = self.len_utf8() - 1;

        let bs = len.view_bits::<Lsb0>();

        destination.push(bs[0]);
        destination.push(bs[1]);

        for b in self.to_string().into_bytes() {
            destination.extend(b.view_bits::<Lsb0>())
        }

        Ok(())
    }
}

impl BitSerdeDeserialization for char {
    fn deserialize_from(mut data: &BitSlice<u8, Lsb0>) -> (&BitSlice<u8, Lsb0>, Self) {
        let mut len: usize = data.chunks(2).next().unwrap().load_le();
        len += 1;

        let bs = &data[2..(len * 8 + 2)];

        let mut bytes: Vec<u8> = Vec::with_capacity(len);
        for c in bs.chunks(8) {
            let val: u8 = c.load();
            bytes.push(val);
        }

        let val = std::str::from_utf8(&bytes).unwrap().chars().next().unwrap();

        data = &data[(len * 8 + 2)..];
        (data, val)
    }
}

pub trait BitSerdeDeserializationMax
where
    Self: Sized,
{
    fn deserialize_with_max(data: &Vec<u8>, max: usize) -> Self {
        let bs = data.view_bits::<Lsb0>();

        BitSerdeDeserializationMax::deserialize_from_with_max(bs, max).1
    }
    fn deserialize_from_with_max(
        data: &BitSlice<u8, Lsb0>,
        max: usize,
    ) -> (&BitSlice<u8, Lsb0>, Self);
}
pub trait BitSerdeSerializationMax {
    fn serialize_with_max(&self, max: usize) -> std::io::Result<Vec<u8>> {
        let mut destination = bitvec!(u8,Lsb0;);
        destination.force_align();

        self.write_bits_to_with_max(&mut destination, max)?;
        destination.force_align();

        Ok(destination.into())
    }
    fn write_bits_to_with_max(
        &self,
        destination: &mut BitVec<u8, Lsb0>,
        max: usize,
    ) -> std::io::Result<()>;
}

pub fn compute_size(max: usize) -> usize {
    ((max + 1) as f64).log2().ceil() as usize
}

macro_rules! implement_trait_with_constraint {
    ($($t:ty),*) => {
        $(
            impl BitSerdeSerializationMax for $t { 
                fn write_bits_to_with_max(&self, destination: &mut BitVec<u8,Lsb0>,max:usize) -> std::io::Result<()> {

                    let size =  {
                        let bits = compute_size(max);

                        let type_size = std::mem::size_of::<$t>();
                        if (bits) > (type_size * 8) {
                            type_size * 8
                        }
                        else {
                            bits
                        }
                    };

                    let bs = self.to_le_bytes();
                    let bs = bs.view_bits::<Lsb0>();

                    let bs = bs.chunks(size).next().unwrap();

                    destination.extend(bs);

                    Ok(())
                }
            }

            impl BitSerdeDeserializationMax for $t { 
                fn deserialize_from_with_max(mut data: &BitSlice<u8,Lsb0>, max: usize) -> (&BitSlice<u8, Lsb0>,Self) {

                    let size =  {
                        let bits = compute_size(max);

                        let type_size = std::mem::size_of::<$t>();
                        if (bits) > (type_size * 8) {
                            type_size * 8
                        }
                        else {
                            bits
                        }
                    };

                    let mut bv = data[..size].to_bitvec();
                    bv.force_align();

                    let val : $t = bv.load_le();

                    data = &data[size..];

                    (data, val)
                }
            }
        )*
    };
}

implement_trait_with_constraint!(u8, u16, u32, u64, u128);

impl<E: BitSerdeSerialization> BitSerdeSerializationMax for Vec<E> {
    fn write_bits_to_with_max(
        &self,
        destination: &mut BitVec<u8, Lsb0>,
        max: usize,
    ) -> std::io::Result<()> {
        let len = self.len();

        let size = {
            let bits = compute_size(max);

            let type_size = std::mem::size_of::<usize>();
            if (bits) > (type_size * 8) {
                type_size * 8
            } else {
                bits
            }
        };

        let bs = len.to_le_bytes();
        let bs = bs.view_bits::<Lsb0>();

        let bs = bs.chunks(size).next().unwrap();

        destination.extend(bs);

        for el in self.iter() {
            el.write_bits_to(destination)?;
        }

        Ok(())
    }
}
impl<E: BitSerdeDeserialization> BitSerdeDeserializationMax for Vec<E> { 
    fn deserialize_from_with_max(
        mut data: &BitSlice<u8, Lsb0>,
        max: usize,
    ) -> (&BitSlice<u8, Lsb0>, Self) {
        let size = {
            let bits = compute_size(max);

            let type_size = std::mem::size_of::<usize>();
            if (bits) > (type_size * 8) {
                type_size * 8
            } else {
                bits
            }
        };

        let bs = &data[0..size];

        let len: usize = bs.load_le();

        let mut vec = Vec::<E>::with_capacity(len);

        data = &data[size..];

        for _ in 0..len {
            let result = BitSerdeDeserialization::deserialize_from(data);

            vec.push(result.1);
            data = result.0;
        }

        (data, vec)
    }
}

impl BitSerdeSerializationMax for String {
    fn write_bits_to_with_max(
        &self,
        destination: &mut BitVec<u8, Lsb0>,
        max: usize,
    ) -> std::io::Result<()> {
        let vec: Vec<u8> = self.as_bytes().into();
        vec.write_bits_to_with_max(destination, max)?;
        Ok(())
    }
}
impl BitSerdeDeserializationMax for String { 
    fn deserialize_from_with_max(
        mut data: &BitSlice<u8, Lsb0>,
        max: usize,
    ) -> (&BitSlice<u8, Lsb0>, Self) {
        let parts: (&BitSlice<u8, Lsb0>, Vec<u8>) =
            BitSerdeDeserializationMax::deserialize_from_with_max(data, max);
        data = parts.0;

        (data, String::from_utf8(parts.1).unwrap())
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
