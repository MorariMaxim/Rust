Going to implement serialize
Going to implement deserialize
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::arch::x86_64::_MM_FROUND_NINT;
use bit_serde_macro::bit_serde;
use bit_serde_trait::compute_size;
use bit_serde_trait::BitSerdeDeserialization;
use bit_serde_trait::BitSerdeDeserializationMax;
use bit_serde_trait::BitSerdeSerialization;
use bit_serde_trait::BitSerdeSerializationMax;
use bitvec::bitvec;
use bitvec::prelude::*;
fn main() -> std::io::Result<()> {
    let test_instance = TestStruct {
        name: String::from("John Doe"),
        gender: true,
        friends: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([String::from("Alice"), String::from("Bob")]),
        ),
        age: 25,
    };
    Ok(())
}
