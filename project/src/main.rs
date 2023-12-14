use std::collections::btree_map::VacantEntry;

use std::str::from_boxed_utf8_unchecked;

use bit_serde_trait::bit_serde;
use bit_serde_trait::BitSerdeDeserialization;
use bit_serde_trait::BitSerdeSerialization;

/*#[bit_serde(Deserialize,Serialize)]
struct myStr {
    #[max = 100]
    id: i32,
    name: String
}
 */
extern crate byteorder;
use bitvec::view::BitView;
use byteorder::ReadBytesExt;
use byteorder::{BigEndian, ByteOrder, LittleEndian};

use bitvec::prelude::*;

use std::mem;
fn main() -> std::io::Result<()> {
    let num = 32.5f64;
    let bytes: Vec<u8> = num.serialize()?;
    //let num =  <f32>::deserialize(&bytes);

    let bytes: BitVec<u8, Lsb0> = BitVec::from_vec(bytes);

    let num = <f64>::deserialize_from(&bytes);

    println!("num = {num}");
    Ok(())
}
