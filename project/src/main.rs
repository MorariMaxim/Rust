use bit_serde_macro::bit_serde;
use bit_serde_trait::BitSerdeDeserialization;
use bit_serde_trait::BitSerdeDeserializationMax;
use bit_serde_trait::BitSerdeSerialization;
use bit_serde_trait::BitSerdeSerializationMax; 

#[bit_serde(Serialize, Deserialize)]
#[derive(Debug)]
struct TestStruct {
    name: String,
    gender: bool,
    friends: Vec<String>,
    #[max = 130]
    age: u128,
    #[max = 15]
    filler: u128,
    str: TestStruct2,
    clas: Class,
}
#[derive(Default,Debug)]
#[bit_serde(Deserialize, Serialize)]
struct TestStruct2 {
    #[max = 15]
    _num1: u128,
    #[max = 15]
    _num2: u128,
    #[max = 15]
    _num3: u128,
    #[max = 15]
    _num4: u128,
}

#[bit_serde(Serialize, Deserialize)]
#[derive(Debug)]
enum Class {
    Warrior,
    Assassin,
    Archer,
    Wizard,
    Priest,
    Evocator,
}

use bitvec::prelude::*;
fn main() -> std::io::Result<()> {
    let test_instance2 = TestStruct2 {
        _num1 : 2,
        _num2 :4,
        _num3 : 8,
        _num4 : 15
    };


    let serialized_obj: Vec<u8> = test_instance2.serialize()?;

    println!("Size of serialized struct = {}", serialized_obj.len());
    //  3<log(15)<4, 4*4 = 16 = 2 bytes, without max attributes, the size is 16 * 4 = 64 bytes

    let test_instance = TestStruct {
        name: String::from("John Doe"),
        gender: true,
        friends: vec![String::from("Alice"), String::from("Bob")],
        age: 25, 
        filler: 2,
        str: test_instance2,
        clas: Class::Warrior
    };
    let serialized_obj: Vec<u8> = test_instance.serialize()?;
    /*
    the len of string/vec is encoded with 8 bytes(usize),
    size of name = 8 bytes + len(name) = 8+8=16
    size of friends = 8 bytes(for vec len) + 16( 8 bytes per string ) + 8(len(AlicBob)) = 32
    size of age = log(130) = 1 byte
    size of str = size of TestStruct 2 = 2 bytes
    size of gender = 1 bit    
    size of filler  = log(15) = 4 bits
    size of clas = log(6) = 3 bits

    total size = 51 bytes, 8 bits, that is 52 bytes
     */
    println!("Size of serialized struct = {}", serialized_obj.len());

    let deserialzed_obj: TestStruct = BitSerdeDeserialization::deserialize(&serialized_obj);

    deserialzed_obj.print_fields();
    Ok(())
}

impl TestStruct {
    fn print_fields(&self) {    
        
        println!("{:?}",self.name);
        println!("{:?}",self.gender);
        println!("{:?}",self.friends);
        println!("{:?}",self.age); 
        println!("{:?}",self.filler);
        println!("{:?}",self.str);
        println!("{:?}",self.clas);
    }
}

/* let test_instance2 = TestStruct2::default();

let serialized_obj: Vec<u8> = test_instance2.serialize()?;

println!("Size of serialized struct = {}",serialized_obj.len());
//  3<log(15)<4, 4*4 = 16 = 2 bytes, without max attributes, the size is 16 * 4 = 64 bytes



let test_instance = TestStruct {
    name: String::from("John Doe"),
    gender: true,
    friends: vec![String::from("Alice"), String::from("Bob")],
    age: 25,
    filler1 : 1,
    filler : 2,
    str : TestStruct2::default()
};
let serialized_obj: Vec<u8> = test_instance.serialize()?;
/*
the len of string/vec is encoded with 8 bytes(usize),
size of name = 8 bytes + len(name) = 8+8=16
size of friends = 8 bytes(for vec len) + 16( 8 bytes per string ) + 8(len(AlicBob)) = 32
size of age = log(130) = 1 byte
size of str = size of TestStruct 2 = 2 bytes
size of gender = 1 bit
size of filler 1 = log(7) = 3 bits
size of filler 2 = log(15) = 4 bits

total size = 51 bytes, 8 bits, that is 52 bytes
 */
println!("Size of serialized struct = {}",serialized_obj.len());

let deserialzed_obj: TestStruct = BitSerdeDeserialization::deserialize(&serialized_obj);

deserialzed_obj.print_fields(); */
