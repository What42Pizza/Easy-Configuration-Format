/// Describes the layout of a loaded settings file line-by-line
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutEntry {
	/// Empty line
	Empty,
	/// Key-value pair
	Key (String),
	/// Comment
	Comment (String),
}



/// Represents a setting value
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
	/// Intentionally empty value
	Empty,
	/// Int value
	I64 (i64),
	/// Float value
	F64 (f64),
	/// Bool value
	Bool (bool),
	/// String value, can be any number of lines
	String (String),
}

impl Value {
	/// Used for formatting ecf files
	pub fn format(&self) -> String {
		match self {
			Self::Empty => String::from("empty"),
			Self::I64 (i64_value) => i64_value.to_string(),
			Self::F64 (f64_value) => f64_value.to_string(),
			Self::Bool (true) => String::from("true"),
			Self::Bool (false) => String::from("false"),
			Self::String (string_value) => {
				if string_value.contains("\n") {
					let mut output = String::from("\"\n");
					for line in string_value.split('\n') {
						output.push('"');
						output += line;
						output.push('\n');
					}
					output
				} else {
					format!("\"{string_value}\"")
				}
			}
		}
	}
	/// Returns "Empty", "String", "Int", "Float", or "Bool" according to enum state
	pub const fn type_as_string(&self) -> &'static str {
		match self {
			Self::Empty => "Empty",
			Self::I64 (_) => "Int",
			Self::F64 (_) => "Float",
			Self::Bool (_) => "Bool",
			Self::String (_) => "String",
		}
	}
	/// Returns "an Empty", "a String", "an Int", "a Float", or "a Bool" according to enum state
	pub const fn type_as_singular_string(&self) -> &'static str {
		match self {
			Self::Empty => "an Empty",
			Self::I64 (_) => "an Int",
			Self::F64 (_) => "a Float",
			Self::Bool (_) => "a Bool",
			Self::String (_) => "a String",
		}
	}
}



/// Output type for `File::from_str`
pub type DidRunUpdaters = bool;
