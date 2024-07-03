#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutEntry {
	Empty,
	Key (String),
	Comment (String),
}



#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
	Empty,
	String (String),
	I64 (i64),
	F64 (f64),
	Bool (bool),
}

impl Value {
	pub fn format(&self) -> String {
		match self {
			Self::Empty => String::from("empty"),
			Self::String (string_value) => format!("\"{string_value}\""),
			Self::I64 (i64_value) => i64_value.to_string(),
			Self::F64 (f64_value) => f64_value.to_string(),
			Self::Bool (true) => String::from("true"),
			Self::Bool (false) => String::from("false"),
		}
	}
}



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParseEntryError {
	pub line: usize,
	pub message: String,
}

impl ParseEntryError {
	pub fn new(line: usize, message: impl Into<String>) -> Self {
		Self {
			line,
			message: message.into(),
		}
	}
}

impl std::error::Error for ParseEntryError {}

impl std::fmt::Display for ParseEntryError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Invalid configuration entry at line {}: {}", self.line + 1, self.message)
	}
}



#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FormatEntryError {
	pub missing_key: String,
}

impl FormatEntryError {
	pub fn new(missing_key: impl Into<String>) -> Self {
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
