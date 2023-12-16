#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use bit_serde_macro::bit_serde;
use bit_serde_trait::BitSerdeDeserialization;
use bit_serde_trait::BitSerdeDeserializationMax;
use bit_serde_trait::BitSerdeSerialization;
use bit_serde_trait::BitSerdeSerializationMax;
struct TestStruct {
    name: String,
    gender: bool,
    friends: Vec<String>,
    age: u128,
    filler: u128,
    str: TestStruct2,
    clas: Class,
}
#[automatically_derived]
impl ::core::fmt::Debug for TestStruct {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        let names: &'static _ = &[
            "name",
            "gender",
            "friends",
            "age",
            "filler",
            "str",
            "clas",
        ];
        let values: &[&dyn ::core::fmt::Debug] = &[
            &self.name,
            &self.gender,
            &self.friends,
            &self.age,
            &self.filler,
            &self.str,
            &&self.clas,
        ];
        ::core::fmt::Formatter::debug_struct_fields_finish(
            f,
            "TestStruct",
            names,
            values,
        )
    }
}
impl BitSerdeSerialization for TestStruct {
    fn write_bits_to(&self, destination: &mut BitVec<u8, Lsb0>) -> std::io::Result<()> {
        self.name.write_bits_to(destination)?;
        self.gender.write_bits_to(destination)?;
        self.friends.write_bits_to(destination)?;
        if (self.age as usize) > 130usize {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Custom I/O error message",
                ),
            );
        }
        self.age.write_bits_to_with_max(destination, 130usize)?;
        if (self.filler as usize) > 15usize {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Custom I/O error message",
                ),
            );
        }
        self.filler.write_bits_to_with_max(destination, 15usize)?;
        self.str.write_bits_to(destination)?;
        self.clas.write_bits_to(destination)?;
        Ok(())
    }
}
impl BitSerdeDeserialization for TestStruct {
    fn deserialize(data: &Vec<u8>) -> Self {
        let bs = data.view_bits::<Lsb0>();
        BitSerdeDeserialization::deserialize_from(bs).1
    }
    fn deserialize_from(mut data: &BitSlice<u8, Lsb0>) -> (&BitSlice<u8, Lsb0>, Self) {
        let parts: (&BitSlice<u8, Lsb0>, String) = BitSerdeDeserialization::deserialize_from(
            &data,
        );
        data = parts.0;
        let name_temp = parts.1;
        let parts: (&BitSlice<u8, Lsb0>, bool) = BitSerdeDeserialization::deserialize_from(
            &data,
        );
        data = parts.0;
        let gender_temp = parts.1;
        let parts: (&BitSlice<u8, Lsb0>, Vec<String>) = BitSerdeDeserialization::deserialize_from(
            &data,
        );
        data = parts.0;
        let friends_temp = parts.1;
        let parts: (&BitSlice<u8, Lsb0>, u128) = BitSerdeDeserializationMax::deserialize_from_with_max(
            &data,
            130usize,
        );
        data = parts.0;
        let age_temp = parts.1;
        let parts: (&BitSlice<u8, Lsb0>, u128) = BitSerdeDeserializationMax::deserialize_from_with_max(
            &data,
            15usize,
        );
        data = parts.0;
        let filler_temp = parts.1;
        let parts: (&BitSlice<u8, Lsb0>, TestStruct2) = BitSerdeDeserialization::deserialize_from(
            &data,
        );
        data = parts.0;
        let str_temp = parts.1;
        let parts: (&BitSlice<u8, Lsb0>, Class) = BitSerdeDeserialization::deserialize_from(
            &data,
        );
        data = parts.0;
        let clas_temp = parts.1;
        let the_object = Self {
            name: name_temp,
            gender: gender_temp,
            friends: friends_temp,
            age: age_temp,
            filler: filler_temp,
            str: str_temp,
            clas: clas_temp,
        };
        (data, the_object)
    }
}
struct TestStruct2 {
    _num1: u128,
    _num2: u128,
    _num3: u128,
    _num4: u128,
}
impl BitSerdeSerialization for TestStruct2 {
    fn write_bits_to(&self, destination: &mut BitVec<u8, Lsb0>) -> std::io::Result<()> {
        if (self._num1 as usize) > 15usize {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Custom I/O error message",
                ),
            );
        }
        self._num1.write_bits_to_with_max(destination, 15usize)?;
        if (self._num2 as usize) > 15usize {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Custom I/O error message",
                ),
            );
        }
        self._num2.write_bits_to_with_max(destination, 15usize)?;
        if (self._num3 as usize) > 15usize {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Custom I/O error message",
                ),
            );
        }
        self._num3.write_bits_to_with_max(destination, 15usize)?;
        if (self._num4 as usize) > 15usize {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Custom I/O error message",
                ),
            );
        }
        self._num4.write_bits_to_with_max(destination, 15usize)?;
        Ok(())
    }
}
impl BitSerdeDeserialization for TestStruct2 {
    fn deserialize(data: &Vec<u8>) -> Self {
        let bs = data.view_bits::<Lsb0>();
        BitSerdeDeserialization::deserialize_from(bs).1
    }
    fn deserialize_from(mut data: &BitSlice<u8, Lsb0>) -> (&BitSlice<u8, Lsb0>, Self) {
        let parts: (&BitSlice<u8, Lsb0>, u128) = BitSerdeDeserializationMax::deserialize_from_with_max(
            &data,
            15usize,
        );
        data = parts.0;
        let _num1_temp = parts.1;
        let parts: (&BitSlice<u8, Lsb0>, u128) = BitSerdeDeserializationMax::deserialize_from_with_max(
            &data,
            15usize,
        );
        data = parts.0;
        let _num2_temp = parts.1;
        let parts: (&BitSlice<u8, Lsb0>, u128) = BitSerdeDeserializationMax::deserialize_from_with_max(
            &data,
            15usize,
        );
        data = parts.0;
        let _num3_temp = parts.1;
        let parts: (&BitSlice<u8, Lsb0>, u128) = BitSerdeDeserializationMax::deserialize_from_with_max(
            &data,
            15usize,
        );
        data = parts.0;
        let _num4_temp = parts.1;
        let the_object = Self {
            _num1: _num1_temp,
            _num2: _num2_temp,
            _num3: _num3_temp,
            _num4: _num4_temp,
        };
        (data, the_object)
    }
}
#[automatically_derived]
impl ::core::default::Default for TestStruct2 {
    #[inline]
    fn default() -> TestStruct2 {
        TestStruct2 {
            _num1: ::core::default::Default::default(),
            _num2: ::core::default::Default::default(),
            _num3: ::core::default::Default::default(),
            _num4: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for TestStruct2 {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "TestStruct2",
            "_num1",
            &self._num1,
            "_num2",
            &self._num2,
            "_num3",
            &self._num3,
            "_num4",
            &&self._num4,
        )
    }
}
enum Class {
    Warrior,
    Assassin,
    Archer,
    Wizard,
    Priest,
    Evocator,
}
#[automatically_derived]
impl ::core::clone::Clone for Class {
    #[inline]
    fn clone(&self) -> Class {
        match self {
            Class::Warrior => Class::Warrior,
            Class::Assassin => Class::Assassin,
            Class::Archer => Class::Archer,
            Class::Wizard => Class::Wizard,
            Class::Priest => Class::Priest,
            Class::Evocator => Class::Evocator,
        }
    }
}
#[automatically_derived]
impl ::core::fmt::Debug for Class {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::write_str(
            f,
            match self {
                Class::Warrior => "Warrior",
                Class::Assassin => "Assassin",
                Class::Archer => "Archer",
                Class::Wizard => "Wizard",
                Class::Priest => "Priest",
                Class::Evocator => "Evocator",
            },
        )
    }
}
impl BitSerdeSerialization for Class {
    fn write_bits_to(&self, destination: &mut BitVec<u8, Lsb0>) -> std::io::Result<()> {
        let val = ((*self).clone() as usize) as u128;
        val.write_bits_to_with_max(destination, 5usize)?;
        Ok(())
    }
}
impl BitSerdeDeserialization for Class {
    fn deserialize(data: &Vec<u8>) -> Self {
        let bs = data.view_bits::<Lsb0>();
        BitSerdeDeserialization::deserialize_from(bs).1
    }
    fn deserialize_from(
        mut data: &BitSlice<u8, Lsb0>,
    ) -> (&bitvec::slice::BitSlice<u8>, Class) {
        let parts: (&BitSlice<u8, Lsb0>, u128) = BitSerdeDeserializationMax::deserialize_from_with_max(
            &data,
            5usize,
        );
        data = parts.0;
        let variant = match parts.1 as usize {
            0usize => Class::Warrior,
            1usize => Class::Assassin,
            2usize => Class::Archer,
            3usize => Class::Wizard,
            4usize => Class::Priest,
            5usize => Class::Evocator,
            _ => {
                {
                    ::core::panicking::panic_fmt(format_args!("something went wrong"));
                };
            }
        };
        (data, variant)
    }
}
use bitvec::prelude::*;
fn main() -> std::io::Result<()> {
    let test_instance2 = TestStruct2 {
        _num1: 2,
        _num2: 4,
        _num3: 8,
        _num4: 16,
    };
    let serialized_obj: Vec<u8> = test_instance2.serialize()?;
    {
        ::std::io::_print(
            format_args!("Size of serialized struct = {0}\n", serialized_obj.len()),
        );
    };
    let test_instance = TestStruct {
        name: String::from("John Doe"),
        gender: true,
        friends: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([String::from("Alice"), String::from("Bob")]),
        ),
        age: 25,
        filler: 2,
        str: test_instance2,
        clas: Class::Warrior,
    };
    let serialized_obj: Vec<u8> = test_instance.serialize()?;
    {
        ::std::io::_print(
            format_args!("Size of serialized struct = {0}\n", serialized_obj.len()),
        );
    };
    let deserialzed_obj: TestStruct = BitSerdeDeserialization::deserialize(
        &serialized_obj,
    );
    deserialzed_obj.print_fields();
    Ok(())
}
impl TestStruct {
    fn print_fields(&self) {
        {
            ::std::io::_print(format_args!("{0:?}\n", self.name));
        };
        {
            ::std::io::_print(format_args!("{0:?}\n", self.gender));
        };
        {
            ::std::io::_print(format_args!("{0:?}\n", self.friends));
        };
        {
            ::std::io::_print(format_args!("{0:?}\n", self.age));
        };
        {
            ::std::io::_print(format_args!("{0:?}\n", self.filler));
        };
        {
            ::std::io::_print(format_args!("{0:?}\n", self.str));
        };
        {
            ::std::io::_print(format_args!("{0:?}\n", self.clas));
        };
    }
}
