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
/// 
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



/// Errors when trying to retrieve settings
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RetrieveSettingError {
	/// Error for attempting to retrieve a non-existent setting key
	Missing {
		/// The key that was queried and is missing
		key: String,
	},
	/// Error for attempting to retrieve a setting value as one type when it is storing another type
	WrongSingularType {
		/// The key that was queried
		key: String,
		/// The expected type of the key's value
		expected: String,
		/// The encountered type of the key's value
		encountered: String,
	},
	/// Error for attempting to retrieve a setting value as a certain set of types when it is storing another type
	WrongMultipleType {
		/// The key that was queried
		key: String,
		/// The expected types of the key's value
		expected: Vec<String>,
		/// The encountered type of the key's value
		encountered: String,
	},
}

impl RetrieveSettingError {
	pub(crate) fn new_missing(key: impl Into<String>) -> Self {
		Self::Missing { key: key.into() }
	}
	pub(crate) fn new_wrong_singular_type(key: impl Into<String>, expected: impl Into<String>, encountered: impl Into<String>) -> Self {
		Self::WrongSingularType { key: key.into(), expected: expected.into(), encountered: encountered.into() }
	}
	pub(crate) fn new_wrong_multiple_type(key: impl Into<String>, expected: Vec<String>, encountered: impl Into<String>) -> Self {
		Self::WrongMultipleType { key: key.into(), expected, encountered: encountered.into() }
	}
}

impl std::error::Error for RetrieveSettingError {}

impl std::fmt::Display for RetrieveSettingError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Missing { key } => write!(f, "Setting '{key}' does not exist"),
			Self::WrongSingularType { key, expected, encountered } => write!(f, "Setting '{key}' was expected to be of type '{expected}', but is of type '{encountered}'"),
			Self::WrongMultipleType { key, expected, encountered } => {
				write!(f, "Setting '{key}' was expected to be of type ")?;
				match expected.len() {
					0 => unreachable!(),
					1 => write!(f, "{}", expected[0])?,
					2 => write!(f, "{} or {}", expected[0], expected[1])?,
					n => {
						for expected_type in expected.iter().take(n - 1) {
							write!(f, "{expected_type}, ")?;
						}
						write!(f, "or {}", expected[n - 1])?;
					}
				}
				write!(f, " but found type '{encountered}'")?;
				Ok(())
			}
		}
	}
}
