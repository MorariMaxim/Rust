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
    let vec = vec![13u8, 15, 17, 19];

    let mut bv = bitvec!(u8,Lsb0;);
    
    vec.write_bits_to(&mut bv)?;

    let vec = bv.into_vec();
    let bs = vec.view_bits::<Lsb0>();

    let vec : Vec<u8> = BitSerdeDeserialization::deserialize_from(&bs).1;

    println!("{:?}",vec);
    Ok(())
}
