## BitSerde

A Rust project that serializes structures and unions at the bit level with maximum efficiency (at least I think it does). 

## How it works

I wrote a **procedural macro** that creates functions for serializing/deserializing a structure/union. 
This means, during compile time the code of the structure is processed, and new code is inserted in its place.
Using the rust crates **syn** and **quote** I can analyse the structure in case and write the respective functions for it.

To apply the macro, write **#[bit_serde(Serialize, Deserialize)]** above the structure.

In src\main.rs is given a comprehensive example and explanations.
