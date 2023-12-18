use bit_serde_macro::bit_serde;
use bit_serde_trait::BitSerdeDeserialization;
use bit_serde_trait::BitSerdeDeserializationMax;
use bit_serde_trait::BitSerdeSerialization;
use bit_serde_trait::BitSerdeSerializationMax;

#[bit_serde(Serialize, Deserialize)]
#[derive(Debug)]
struct TestStruct {
    #[max = 255]
    name: String,
    gender: bool,
    #[max = 255]
    friends: Vec<NameStruct>,
    #[max = 130]
    age: u128,
    #[max = 3]
    filler: u128,
    clas: Class,
    favourite_char : char
}
#[bit_serde(Serialize, Deserialize)]
#[derive(Debug)]
struct NameStruct {
    #[max = 255]
    val: String,
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

    let test_instance = TestStruct {
        name: String::from("John Doe"),
        gender: true,
        friends: vec![
            NameStruct {
                val: String::from("Alice"),
            },
            NameStruct {
                val: String::from("Bob"),
            },
        ],
        age: 25,
        filler: 2,
        clas: Class::Warrior,
        favourite_char : '🦀'
    };
    let  serialized_obj: Vec<u8> = test_instance.serialize()?;
    //`let mut serialized_obj: Vec<u8>  = vec![0u8,1,2,3].into();
    /*
    the max attribute above a string/vec indicates its maximum length, so that we don't have to usze usize (8 bytes) to encode its length
    
    size of name:   
        its maximum length is 255, log(255+1) = 8, so 1 byte for its len
        the content of the string itself is 8 bytes
        size of name = 9
    
    so far 9 bytes

    size of friends:
        similar to name, its len = 1 byte
        now, we have to wrap the content of its elements in another struct, because max attribute applies only 
        to the len of the vector, not the strings in it.
        the size of a NameStruct is 1 byte + string.len()
        the size of friends = 3 bytes + 8 bytes ("AliceBob") = 11 bytes

    so far 20 bytes

    size of age = 1 byte, as 7 <log(13) < 8

    so far 21 bytes

    size of filler is 2 bits

    size of clas is 3 bits

    size of gender is 1 bi

    to encode a character, we use 2 bits to encode its length (an utf8 character has at maximum 4 bytes) and the bytes themselves

    size of favourite_char is 4 bytes, 2 bits, 
    
    25 bytes + 8 bits = 26 bytes
     */
 
    println!("Size of serialized struct = {}", serialized_obj.len());
    
    let deserialzed_obj: TestStruct = BitSerdeDeserialization::deserialize(&serialized_obj)?;

    deserialzed_obj.print_fields();

    Ok(())
}

impl TestStruct {
    fn print_fields(&self) {
        println!("{:?}", self.name);
        println!("{:?}", self.gender);
        println!("{:?}", self.friends);
        println!("{:?}", self.age);
        println!("{:?}", self.filler);
        println!("{:?}", self.clas);
        println!("{:?}", self.favourite_char);
    }
}
