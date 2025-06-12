use std::{collections::HashMap, ops::{Deref, DerefMut}};



/// Holds data for a file's contents, layout, and version
#[derive(Debug, Clone, PartialEq)]
pub struct File {
	/// Contents of file
	pub values: HashMap<String, Value>,
	/// Layout of file
	pub layout: Vec<LayoutEntry>,
	/// Version of file (strongly recommended to hold the latest version of settings that your application supports)
	pub version: usize,
}

impl Deref for File {
	type Target = HashMap<String, Value>;
	fn deref(&self) -> &Self::Target {
		&self.values
	}
}

impl DerefMut for File {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.values
	}
}



/// Describes the layout of a loaded settings file
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
	/// String value, can be any number of lines
	String (String),
	/// Int value
	I64 (i64),
	/// Float value
	F64 (f64),
	/// Bool value
	Bool (bool),
}

impl Value {
	/// Used for formatting settings
	pub fn format(&self) -> String {
		match self {
			Self::Empty => String::from("empty"),
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
			Self::I64 (i64_value) => i64_value.to_string(),
			Self::F64 (f64_value) => f64_value.to_string(),
			Self::Bool (true) => String::from("true"),
			Self::Bool (false) => String::from("false"),
		}
	}
	/// Returns "Empty", "String", "Int", "Float", or "Bool" according to enum state
	pub const fn type_as_string(&self) -> &'static str {
		match self {
			Self::Empty => "Empty",
			Self::String (_) => "String",
			Self::I64 (_) => "Int",
			Self::F64 (_) => "Float",
			Self::Bool (_) => "Bool",
		}
	}
	/// Returns "an Empty", "a String", "an Int", "a Float", or "a Bool" according to enum state
	pub const  fn type_as_singular_string(&self) -> &'static str {
		match self {
			Self::Empty => "an Empty",
			Self::String (_) => "a String",
			Self::I64 (_) => "an Int",
			Self::F64 (_) => "a Float",
			Self::Bool (_) => "a Bool",
		}
	}
}



/// Errors while parsing settings
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParseEntryError {
	/// Line number of invalid entry (using 1-based indexing)
	pub line: usize,
	/// Error message / reason for being invalid
	pub message: String,
}

impl ParseEntryError {
	pub(crate) fn new(raw_line: usize, message: impl Into<String>) -> Self {
		Self {
			line: raw_line + 1,
			message: message.into(),
		}
	}
}

impl std::error::Error for ParseEntryError {}

impl std::fmt::Display for ParseEntryError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Invalid configuration entry at line {}: {}", self.line, self.message)
	}
}



/// Errors while formatting settings
/// Right now the only error that can occur is having a key specified in the layout that isn't defined in the settings hashmap
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FormatEntryError {
	/// Name of missing key
	pub missing_key: String,
}

impl FormatEntryError {
	pub(crate) fn new(missing_key: impl Into<String>) -> Self {
		Self {
			missing_key: missing_key.into(),
		}
	}
}

impl std::error::Error for FormatEntryError {}

impl std::fmt::Display for FormatEntryError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Failed to format configuration entry, no value found for key {}", self.missing_key)
	}
}
