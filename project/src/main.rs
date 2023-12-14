use std::collections::btree_map::VacantEntry;

use bit_serde_trait::bit_serde;
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
use byteorder::{BigEndian, ByteOrder, LittleEndian};

use bitvec::prelude::*;

fn main() {
    use bitvec::prelude::*;

    let mut bv = bitvec![u8, Lsb0;];
    bv.push(true);

    let bs = bv.as_raw_mut_slice();

    let mut chunks = bs.view_bits_mut::<Lsb0>().chunks_mut(8);

    let store = chunks.next().unwrap();

    store.store(128);

    let x = store.load::<u8>();

    println!("{x}");

    let sl = bs.view_bits_mut::<Msb0>();
}
