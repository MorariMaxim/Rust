use std::arch::x86_64::_MM_FROUND_NINT;

use bit_serde_trait::bit_serde;
use bit_serde_trait::compute_size;
use bit_serde_trait::BitSerdeDeserialization;
use bit_serde_trait::BitSerdeDeserializationMax;
use bit_serde_trait::BitSerdeSerialization;
use bit_serde_trait::BitSerdeSerializationMax;
use bitvec::bitvec;

/*#[bit_serde(Deserialize,Serialize)]
struct myStr {
    #[max = 100]
    id: i32,
    name: String
}
*/

#[derive(Default)]
struct TestStruct {
    name: String,
    gender: bool,
    friends: Vec<String>,
    age: u8,
}

impl TestStruct {
    
    fn print_fields(&self){

        println!("Name: {}",self.name);
        if self.gender {
            println!("Gender: Male");
        }
        else  {
            println!("Gender: Female");
        }
        println!("Friends: ");
        for f in self.friends.iter() {
            print!("{} ",f);
        }        
        println!("\nAge: {}",self.age);
    }

}

use bitvec::prelude::*;
fn main() -> std::io::Result<()> {
    let mut bv = bitvec!(u8,Lsb0;);

    let mut obj = TestStruct {
        name: String::from("John Doe"),
        gender: true,
        friends: vec![String::from("Alice"), String::from("Bob")],
        age: 25,
    };

    println!("Initial object");
    obj.print_fields();
    let mut temp_size = 0usize;
    println!("size of serialized obj = {} ", bv.len());
    obj.name.write_bits_to(&mut bv)?;
    println!(
        "size of serialized obj = {}\n diff =  {}",
        bv.len(),
        bv.len() - temp_size
    );
    temp_size = bv.len();
    obj.gender.write_bits_to(&mut bv)?;
    println!(
        "size of serialized obj = {}\n diff =  {}",
        bv.len(),
        bv.len() - temp_size
    );
    temp_size = bv.len();
    obj.friends.write_bits_to(&mut bv)?;
    println!(
        "size of serialized obj = {}\n diff =  {}",
        bv.len(),
        bv.len() - temp_size
    );
    temp_size = bv.len();
    obj.age.write_bits_to(&mut bv)?;
    println!(
        "size of serialized obj = {}\n diff =  {}",
        bv.len(),
        bv.len() - temp_size
    );

    let vec = bv.into_vec();

    let mut bs = vec.view_bits::<Lsb0>();
    
    let mut recovered_obj : TestStruct = TestStruct::default();

    let vec: (&BitSlice<u8, Lsb0>, String) = BitSerdeDeserialization::deserialize_from(&bs);
    bs = vec.0;
    recovered_obj.name = vec.1;

    let vec: (&BitSlice<u8, Lsb0>, bool) = BitSerdeDeserialization::deserialize_from(&bs);
    bs = vec.0;
    recovered_obj.gender = vec.1;

    let vec: (&BitSlice<u8, Lsb0>, Vec<String>) = BitSerdeDeserialization::deserialize_from(&bs);
    bs = vec.0;
    recovered_obj.friends = vec.1;

    let vec: (&BitSlice<u8, Lsb0>, u8) = BitSerdeDeserialization::deserialize_from(&bs);
    //bs = vec.0;
    recovered_obj.age = vec.1;

    println!("Recovered object");
    recovered_obj.print_fields();

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
