use std::arch::x86_64::_MM_FROUND_NINT;

use bit_serde_trait::bit_serde;
use bit_serde_trait::BitSerdeDeserialization;
use bit_serde_trait::BitSerdeSerialization;
use bitvec::bitvec;

/*#[bit_serde(Deserialize,Serialize)]
struct myStr {
    #[max = 100]
    id: i32,
    name: String
}
 */

use bitvec::prelude::*;
fn main() -> std::io::Result<()> {
    let vec1 = vec![13u8, 15, 17, 19];
    let flag1 = true;
    let vec2 = vec![1.3f32, 1.5, 1.7, 1.9];
    let mut bv = bitvec!(u8,Lsb0;);
    let str = String::from("this works!");
    let flag2 = false;
    let c = '€';
    let c2 = '😉';
    
    
    vec1.write_bits_to(&mut bv)?;
    flag1.write_bits_to(&mut bv)?;
    vec2.write_bits_to(&mut bv)?;
    str.write_bits_to(&mut bv)?;
    flag2.write_bits_to(&mut bv)?;
    c.write_bits_to(&mut bv)?;
    c2.write_bits_to(&mut bv)?;
    
    let vec = bv.into_vec();
    let mut bs = vec.view_bits::<Lsb0>();
    
    let vec:(&BitSlice<u8,Lsb0>,Vec<u8>)  = BitSerdeDeserialization::deserialize_from(&bs);
    bs = vec.0;
    println!("{:?}",vec.1);
    
    let vec:(&BitSlice<u8,Lsb0>,bool)  = BitSerdeDeserialization::deserialize_from(&bs);
    bs = vec.0;
    println!("{:?}",vec.1);
    
    let vec:(&BitSlice<u8,Lsb0>,Vec<f32>)  = BitSerdeDeserialization::deserialize_from(&bs);
    bs = vec.0;
    println!("{:?}",vec.1);
    
    let vec:(&BitSlice<u8,Lsb0>,String)  = BitSerdeDeserialization::deserialize_from(&bs);
    bs = vec.0;
    println!("{:?}",vec.1);
    
    let vec:(&BitSlice<u8,Lsb0>,bool)  = BitSerdeDeserialization::deserialize_from(&bs);
    bs = vec.0;
    println!("{:?}",vec.1);
    
    let vec:(&BitSlice<u8,Lsb0>,char)  = BitSerdeDeserialization::deserialize_from(&bs);
    bs = vec.0;
    println!("{:?}",vec.1);
    
    let vec:(&BitSlice<u8,Lsb0>,char)  = BitSerdeDeserialization::deserialize_from(&bs);
    //bs = vec.0;
    println!("{:?}",vec.1);
    Ok(())
}


