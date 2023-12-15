
use bit_serde_macro::bit_serde;
use bit_serde_trait::compute_size;
use bit_serde_trait::BitSerdeDeserialization;
use bit_serde_trait::BitSerdeDeserializationMax;
use bit_serde_trait::BitSerdeSerialization;
use bit_serde_trait::BitSerdeSerializationMax;
use bitvec::bitvec;


#[bit_serde(Deserialize, Serialize)]
struct TestStruct {
    name: String,
    gender: bool,
    friends: Vec<String>,
    age: u8,
} 

use bitvec::prelude::*;
fn main() -> std::io::Result<()> {
    let test_instance = TestStruct {
        name: String::from("John Doe"),
        gender: true,
        friends: vec![String::from("Alice"), String::from("Bob")],
        age: 25,
    };

    let serialized_obj: Vec<u8> = test_instance.serialize()?;

    let deserialzed_obj : TestStruct = BitSerdeDeserialization::deserialize(&serialized_obj);

    deserialzed_obj.print_fields();
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
