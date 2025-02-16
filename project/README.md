
## BitSerde

A Rust project that serializes structures and unions at the bit level with maximum efficiency (at least I think it does). 

## How it works

I wrote a **procedural macro** that creates functions for serializing/deserializing a structure/union. 
This means, during compile time the code of the structure is processed, and new code is inserted in its place.
Using the rust crates **syn** and **quote** I can analyse the structure in case and write the respective functions for it.

To apply the macro, write **#[bit_serde(Serialize, Deserialize)]** on top of a structure.

# Illustration
In src\main.rs is given a comprehensive example and explanations. 
```rust

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
#[bit_serde(Serialize, Deserialize)]
#[derive(Debug)]
struct NameStruct {
    #[max = 255]
    val: String,
}
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
    favourite_char: char,
}

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
    favourite_char: '🦀',
};  
``` 
Let's analyse the size if this TestStruct instance.
The **max** attribute above a string/vector indicates its maximum length, so that we don't have to use **usize** (4/8 bytes, used by Rust for this purpose) to encode its length

- size of **name**:
	- its maximum length is 255, log(255+1) = 8, so 1 byte for its len
	- the content of the string itself is 8 bytes
	- size of name = 9

so far **9** bytes

- size of **friends**:
    - similar to name, its len = 1 byte
    - now, we have to wrap the content of its elements in another struct, because max attribute applies only
    - to the len of the vector, not the strings in it.
    - the size of a NameStruct is 1 byte + string.len()
    - the size of friends = 1(for length of friends) + 1 + 1 (for length of the 2 NameStruct) + 5 + 3 (lengths of 'Alice' and 'Bob') = 11 bytes

so far **20** bytes

size of **age** = 1 byte, as 7 <log(130) < 8

so far **21** bytes

size of **filler** is 2 bits

size of **clas** is 3 bits

size of **gender** is 1 bit

to encode a character, we use 2 bits to encode its length (an utf8 character has at maximum 4 bytes) and the bytes themselves

size of **favourite_char** is 4 bytes, 2 bits,

25 bytes + 8 bits = **26** bytes

Not using this library, its size would be **112** bytes. You can do the computations yourself!

# Conclusion
This library reduces the memory required to store an instance of a practical data type from 112 bytes to just 26 bytes—a reduction of approximately 4.3 times!
