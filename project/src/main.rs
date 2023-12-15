use std::arch::x86_64::_MM_FROUND_NINT;

use bit_serde_trait::bit_serde;
use bit_serde_trait::BitSerdeDeserialization;
use bit_serde_trait::BitSerdeSerialization;
use bit_serde_trait::BitSerdeDeserializationMax;
use bit_serde_trait::BitSerdeSerializationMax;
use bit_serde_trait::compute_size;
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
    
    let mut bv = bitvec!(u8,Lsb0;);

    let n1  = <u64>::MAX;

    n1.write_bits_to_with_max(&mut bv,1)?;
    n1.write_bits_to_with_max(&mut bv,3)?;
    n1.write_bits_to_with_max(&mut bv,7)?;
    n1.write_bits_to_with_max(&mut bv,15)?;
    n1.write_bits_to_with_max(&mut bv,31)?;
    n1.write_bits_to_with_max(&mut bv,63)?;
    n1.write_bits_to_with_max(&mut bv,127)?;
    n1.write_bits_to_with_max(&mut bv,255)?;
    n1.write_bits_to_with_max(&mut bv,511)?;
    n1.write_bits_to_with_max(&mut bv,1023)?;

    bv.force_align();
    let bv = bv.into_vec();
    let mut bs = bv.view_bits::<Lsb0>();

    for (i,b) in bs.iter().enumerate() {
        println!("{} = {}",i,b);
    }

    
 
    for i in 0..10 {
        let val :(&BitSlice<u8,Lsb0>, u64) = BitSerdeDeserializationMax::deserialize_from_with_max(bs, 2f64.powi(i) as usize );
        bs = val.0;
        println!("{i}th. num = {}",val.1 );
    }    



    


    Ok(())
}



/*let vec1 = vec![13u8, 15, 17, 19];
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
    println!("{:?}",vec.1); */