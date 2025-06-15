//! # Easy Configuration Format
//! 
//! ### A settings format that strikes a great balance between usage simplicity and parsing simplicity, with aspects like:
//! - Support for strings, ints, float, bools, and comments
//! - Elegant error handling, an invalid line in the middle won't ruin everything afterwards and loading then saving a file will always result in a valid ecf file (to see this in action, just run `cargo run --example main`)
//! - 'Setting updater' functions have built-in support and encouragement
//! - Almost no code (~500 sloc) and no dependencies (other than std)
//! 
//! <br>
//! 
//! ## Example settings file:
//! 
//! ```txt
//! format 1
//! # This first line defines the version number of your settings file. If you want to update
//! # your program's settings, this will allow you to update users' settings file to your
//! # newer version
//! 
//! example key: "example value"
//! 
//! example blank: empty
//! example string: "not empty"
//! example int: 3
//! example float: 3.5
//! example bool: true
//! example multiline: "
//! "first line (#0)
//! "also, because of how strings are stored, you can have " characters inside a string with
//! "no escape codes needed
//! "last line (#3)
//! example string 2: "you can also put " chars in single-line strings"
//! 
//! example namespace.example key: "example value 2"
//! # "namespaces" are entirely made up, they're just fancy names but it's still the
//! # recommended way to structure settings
//! 
//! # example comment
//! 
//! ##
//! example multiline comment
//! just like strings, you can have extra # chars anywhere you want (as long as you don't 
//! want one of the lines in a comment to just be "##")
//! ##
//! 
//! example array.0: "value 0"
//! example array.1: "value 1"
//! example array.2: "value 2"
//! example array.3: "value 3"
//! 
//! example nested array.0.name: "person 0"
//! example nested array.0.age: "age 0"
//! example nested array.0.friends.0: "person 1"
//! example nested array.0.friends.1: "person 2"
//! 
//! example nested array.1.name: "person 1"
//! example nested array.1.age: "age 1"
//! example nested array.1.friends.0: "person 0"
//! example nested array.1.friends.1: "person 2"
//! 
//! 
//! 
//! # examples for error handling:
//! 
//! example duplicate key: "this key will be kept"
//! example duplicate key: "this key will be commented"
//! 
//! invalid key "doesn't have any colon"
//! invalid value 1: "missing an ending quote
//! invalid value 2: missing a starting quote"
//! invalid value 3: missing both quotes
//! # empty multiline strings aren't allowed:
//! invalid value 4: "
//! 
//! invalid value 6: .3
//! 
//! invalid entry: empty # inline comments aren't allowed
//! 
//! ##
//! invalid multiline comment, only these two lines will be commented because of this
//! 
//! # single-line comments cannot be invalid!
//! 
//! working key: "and even after all that, it can still keep parsing settings!"
//! 
//! ```
//! 
//! ### See the specification [Here](specification.txt)
//! 
//! <br>
//! <br>
//! <br>
//! 
//! A settings file is intended to be represented in code using two main values: the layout vec and the values hashmap. The layout vec describes the layout of the settings file according to how it was when it was parsed, and modifying it at runtime isn't recommended (because there should no need to do so). The values hashmap simply stores the key-value (String, ecf::Value) pairs, and this is what your code will interact with.
//! 
//! Also, I strongly recommend using a format updating system like what's shown in the [example](https://github.com/What42Pizza/Easy-Configuration-Format/blob/main/examples/main.rs).
//! 
//! <br>
//! <br>



#![warn(missing_docs)]

#![feature(anonymous_lifetime_in_impl_trait)]



/// This is the main operation of this crate. An ecf File is an instance of a configuration and its formatting.
pub mod file;
pub use file::*;
/// Miscellaneous data used by the crate
pub mod data;
pub use data::*;
/// All errors defined by the crate
pub mod errors;
pub use errors::*;



/// Slightly easier way to create a new `ecf::Value::Empty`
pub const fn empty() -> Value {
	Value::Empty
}

/// Slightly easier way to create a new `ecf::Value::I64()`
pub const fn i64(v: i64) -> Value {
	Value::I64 (v)
}

/// Slightly easier way to create a new `ecf::Value::I64()`
/// 
/// This isn't marked as `const` because it gives an error for const trait functions
pub fn to_i64(v: impl ToI64) -> Value {
	Value::I64 (v.to_i64())
}

/// Slightly easier way to create a new `ecf::Value::F64()`
pub const fn f64(v: f64) -> Value {
	Value::F64 (v)
}

/// Slightly easier way to create a new `ecf::Value::F64()`
/// 
/// This isn't marked as `const` because it gives an error for const trait functions
pub fn to_f64(v: impl ToF64) -> Value {
	Value::F64 (v.to_f64())
}

/// Slightly easier way to create a new `ecf::Value::Bool()`
pub const fn bool(v: bool) -> Value {
	Value::Bool (v)
}

/// Slightly easier way to create a new `ecf::Value::String()`
/// 
/// This isn't marked as `const` because it works with strings
pub fn string(v: impl ToString) -> Value {
	Value::String (v.to_string())
}



/// Used for `ecf::to_i64()` overloading
pub trait ToI64 {
	/// Purpose of trait
	fn to_i64(self) -> i64;
}

macro_rules! impl_to_i64 {
	($impl_type:ty) => {
		impl ToI64 for $impl_type {
			fn to_i64(self) -> i64 {
				self as i64
			}
		}
	};
}

impl_to_i64!(i8);
impl_to_i64!(u8);
impl_to_i64!(i16);
impl_to_i64!(u16);
impl_to_i64!(i32);
impl_to_i64!(u32);
impl_to_i64!(i64);
impl_to_i64!(u64);
impl_to_i64!(i128);
impl_to_i64!(u128);
impl_to_i64!(isize);
impl_to_i64!(usize);
impl_to_i64!(f32);
impl_to_i64!(f64);



/// Used for `ecf::to_f64()` overloading
pub trait ToF64 {
	/// Purpose of trait
	fn to_f64(self) -> f64;
}

macro_rules! impl_to_f64 {
	($impl_type:ty) => {
		impl ToF64 for $impl_type {
			fn to_f64(self) -> f64 {
				self as f64
			}
		}
	};
}

impl_to_f64!(i8);
impl_to_f64!(u8);
impl_to_f64!(i16);
impl_to_f64!(u16);
impl_to_f64!(i32);
impl_to_f64!(u32);
impl_to_f64!(i64);
impl_to_f64!(u64);
impl_to_f64!(i128);
impl_to_f64!(u128);
impl_to_f64!(isize);
impl_to_f64!(usize);
impl_to_f64!(f32);
impl_to_f64!(f64);
