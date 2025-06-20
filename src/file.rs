use crate::*;
use std::{collections::{HashMap, HashSet}, ops::{Deref, DerefMut}};





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



impl File {
	
	
	
	/// Converts a settings file into a layout + values, opposite of `format_settings()`
	/// 
	/// The generic `T` is for passing generic data to the updater functions
	pub fn from_str<T>(contents: impl AsRef<str>, updater_fns: &[fn(&mut HashMap<String, Value>, &mut T)], args: &mut T) -> (Self, DidRunUpdaters, Vec<ParseEntryError>) {
		let mut layout = vec!();
		let mut values = HashMap::new();
		let mut errors = vec!();
		
		let lines = contents.as_ref().split('\n').collect::<Vec<_>>();
		let version = get_file_version(lines[0].trim());
		let mut line_i = 1;
		loop {
			let result = parse_line(&lines, &mut line_i, &mut layout, &mut values);
			if let Err(err) = result {
				layout.push(LayoutEntry::Comment (lines[line_i].to_string()));
				errors.push(err);
			}
			line_i += 1;
			if line_i >= lines.len() {break;}
		}
		
		let did_run_updaters = if let Some(version) = version {
			let fns_to_run = &updater_fns[version - 1 ..];
			for updater_fn in fns_to_run {
				(updater_fn)(&mut values, args);
			}
			!fns_to_run.is_empty()
		} else {
			errors.push(ParseEntryError::new(0, "Could not find version, assuming version is latest"));
			false
		};
		
		(
			Self {
				values,
				layout,
				version: updater_fns.len() + 1,
			},
			did_run_updaters,
			errors,
		)
	}
	
	
	
	/// Converts a layout plus values into a formatted settings file, opposite of `from_str()`
	pub fn to_str(&self) -> (String, Vec<FormatEntryError>) {
		let mut output = format!("format {}\n", self.version);
		if self.layout.is_empty() {return (output, vec!());}
		let mut errors = vec!();
		let mut printed_keys = HashSet::new();
		for entry in &self.layout {
			match entry {
				LayoutEntry::Empty => {}
				LayoutEntry::Comment (comment) => {
					if comment.contains('\n') {
						output += "##\n";
						output += comment;
						output += "\n##";
					} else {
						output.push('#');
						output += comment;
					}
				}
				LayoutEntry::Key (key) => {
					output += key;
					output += ": ";
					let value = self.get(key);
					if let Some(value) = value {
						output += &value.format();
					} else {
						errors.push(FormatEntryError::new(key));
						continue;
					};
					printed_keys.insert(key.to_string());
				}
			}
			output.push('\n');
		}
		for (key, value) in &self.values {
			if printed_keys.contains(key) {continue;}
			output += key;
			output += ": ";
			output += &value.format();
			output.push('\n');
		}
		output.pop();
		(output, errors)
	}
	
	
	
	/// Basically an assert for a setting key existing but being left empty
	pub fn get_empty(&self, key: impl AsRef<str>) -> Result<(), RetrieveSettingError> {
		let key = key.as_ref();
		match self.get(key) {
			None => Err(RetrieveSettingError::new_missing(key)),
			Some(Value::Empty) => Ok(()),
			Some(value) => Err(RetrieveSettingError::new_wrong_singular_type(key.to_string(), "empty", value.type_as_string())),
		}
	}
	
	/// Gets the value of a setting as an int (or returns an error if the setting is missing or if it's holding the wrong type)
	pub fn get_int(&self, key: impl AsRef<str>) -> Result<i64, RetrieveSettingError> {
		let key = key.as_ref();
		match self.get(key) {
			None => Err(RetrieveSettingError::new_missing(key)),
			Some(Value::I64 (v)) => Ok(*v),
			Some(value) => Err(RetrieveSettingError::new_wrong_singular_type(key.to_string(), "Int", value.type_as_string())),
		}
	}
	
	/// Gets the value of a setting as an int as mut (or returns an error if the setting is missing or if it's holding the wrong type)
	pub fn get_int_mut(&mut self, key: impl AsRef<str>) -> Result<&mut i64, RetrieveSettingError> {
		let key = key.as_ref();
		match self.get_mut(key) {
			None => Err(RetrieveSettingError::new_missing(key)),
			Some(Value::I64 (v)) => Ok(v),
			Some(value) => Err(RetrieveSettingError::new_wrong_singular_type(key.to_string(), "Int", value.type_as_string())),
		}
	}
	
	/// Gets the value of a setting as a float (or returns an error if the setting is missing or if it's holding the wrong type)
	pub fn get_float(&self, key: impl AsRef<str>) -> Result<f64, RetrieveSettingError> {
		let key = key.as_ref();
		match self.get(key) {
			None => Err(RetrieveSettingError::new_missing(key)),
			Some(Value::F64 (v)) => Ok(*v),
			Some(value) => Err(RetrieveSettingError::new_wrong_singular_type(key.to_string(), "Float", value.type_as_string())),
		}
	}
	
	/// Gets the value of a setting as a float as mut (or returns an error if the setting is missing or if it's holding the wrong type)
	pub fn get_float_mut(&mut self, key: impl AsRef<str>) -> Result<&mut f64, RetrieveSettingError> {
		let key = key.as_ref();
		match self.get_mut(key) {
			None => Err(RetrieveSettingError::new_missing(key)),
			Some(Value::F64 (v)) => Ok(v),
			Some(value) => Err(RetrieveSettingError::new_wrong_singular_type(key.to_string(), "Float", value.type_as_string())),
		}
	}
	
	/// Gets the value of a setting as a float, but also allowing ints (or returns an error if the setting is missing or if it's holding the wrong type)
	/// 
	/// This does'n't' have a `get_number_mut()` because there's no return type that would make sense
	pub fn get_number(&self, key: impl AsRef<str>) -> Result<f64, RetrieveSettingError> {
		let key = key.as_ref();
		match self.get(key) {
			None => Err(RetrieveSettingError::new_missing(key)),
			Some(Value::I64 (v)) => Ok(*v as f64),
			Some(Value::F64 (v)) => Ok(*v),
			Some(value) => Err(RetrieveSettingError::new_wrong_multiple_type(key.to_string(), vec!(String::from("Int"), String::from("Float")), value.type_as_string())),
		}
	}
	
	/// Gets the value of a setting as a bool (or returns an error if the setting is missing or if it's holding the wrong type)
	pub fn get_bool(&self, key: impl AsRef<str>) -> Result<bool, RetrieveSettingError> {
		let key = key.as_ref();
		match self.get(key) {
			None => Err(RetrieveSettingError::new_missing(key)),
			Some(Value::Bool (v)) => Ok(*v),
			Some(value) => Err(RetrieveSettingError::new_wrong_singular_type(key.to_string(), "Bool", value.type_as_string())),
		}
	}
	
	/// Gets the value of a setting as a bool as mut (or returns an error if the setting is missing or if it's holding the wrong type)
	pub fn get_bool_mut(&mut self, key: impl AsRef<str>) -> Result<&mut bool, RetrieveSettingError> {
		let key = key.as_ref();
		match self.get_mut(key) {
			None => Err(RetrieveSettingError::new_missing(key)),
			Some(Value::Bool (v)) => Ok(v),
			Some(value) => Err(RetrieveSettingError::new_wrong_singular_type(key.to_string(), "Bool", value.type_as_string())),
		}
	}
	
	/// Gets the value of a setting as a string (or returns an error if the setting is missing or if it's holding the wrong type)
	pub fn get_str(&self, key: impl AsRef<str>) -> Result<&str, RetrieveSettingError> {
		let key = key.as_ref();
		match self.get(key) {
			None => Err(RetrieveSettingError::new_missing(key)),
			Some(Value::String (v)) => Ok(v),
			Some(value) => Err(RetrieveSettingError::new_wrong_singular_type(key.to_string(), "String", value.type_as_string())),
		}
	}
	
	/// Gets the value of a setting as a string as mut (or returns an error if the setting is missing or if it's holding the wrong type)
	pub fn get_string_mut(&mut self, key: impl AsRef<str>) -> Result<&mut String, RetrieveSettingError> {
		let key = key.as_ref();
		match self.get_mut(key) {
			None => Err(RetrieveSettingError::new_missing(key)),
			Some(Value::String (v)) => Ok(v),
			Some(value) => Err(RetrieveSettingError::new_wrong_singular_type(key.to_string(), "String", value.type_as_string())),
		}
	}
	
	
	
	/// Add key-value pairs to the `values` hashmap for keys that aren't set
	pub fn add_missing_values(&mut self, defaults: impl IntoIterator<Item = (&str, Value)>) {
		for (key, value) in defaults {
			if self.contains_key(key) {continue;}
			self.insert(key.to_string(), value);
		}
	}
	
	
	
}





fn get_file_version(first_line: &str) -> Option<usize> {
	let format_str = first_line.strip_prefix("format ")?;
	format_str.parse::<usize>().ok()
}



fn parse_line(
	lines: &[&str],
	line_i: &mut usize,
	layout: &mut Vec<LayoutEntry>,
	values: &mut HashMap<String, Value>,
) -> Result<(), ParseEntryError> {
	
	let line_trimmed = lines[*line_i].trim();
	if line_trimmed.is_empty() {
		layout.push(LayoutEntry::Empty);
		return Ok(());
	}
	
	if line_trimmed == "##" {
		layout.push(parse_multiline_comment(lines, line_i)?);
		return Ok(());
	}
	if let Some(comment) = line_trimmed.strip_prefix("#") {
		layout.push(LayoutEntry::Comment (comment.to_string()));
		return Ok(());
	}
	
	let colon_index = line_trimmed.find(':');
	let Some(colon_index) = colon_index else {return Err(ParseEntryError::new(*line_i, "No colon was found, either add a colon after the key or mark this as a comment."));};
	if colon_index == 0 {return Err(ParseEntryError::new(*line_i, "Lines cannot start with a colon."));}
	let key = &line_trimmed[..colon_index];
	if values.contains_key(key) {return Err(ParseEntryError::new(*line_i, format!("Key \"{key}\" is already defined.")));}
	let value = parse_value(lines, line_i, colon_index)?;
	layout.push(LayoutEntry::Key (key.to_string()));
	values.insert(key.to_string(), value);
	
	Ok(())
}



fn parse_multiline_comment(
	lines: &[&str],
	line_i: &mut usize,
) -> Result<LayoutEntry, ParseEntryError> {
	
	let start_line_i = *line_i;
	let mut output = String::new();
	*line_i += 1;
	while lines[*line_i].trim() != "##" {
		output += lines[*line_i];
		output.push('\n');
		*line_i += 1;
		if *line_i == lines.len() {
			*line_i = start_line_i;
			return Err(ParseEntryError::new(start_line_i, "Could not find an end of this multiline comment. To end a multiline comment, its last line should be nothing but '##'."));
		}
	}
	output.pop();
	Ok(LayoutEntry::Comment (output))
}



fn parse_value(lines: &[&str], line_i: &mut usize, colon_index: usize) -> Result<Value, ParseEntryError> {
	let line_trimmed = lines[*line_i].trim();
	
	let value_start_i =
		line_trimmed.char_indices()
		.skip(colon_index + 1)
		.find(|(_i, c)| !c.is_whitespace());
	let Some((value_start_i, _c)) = value_start_i else {return Err(ParseEntryError::new(*line_i, "No value was found for this key (if this is meant to be empty, please set the value as 'empty')."));};
	
	let value = &line_trimmed[value_start_i..];
	match &*value.to_lowercase() {
		"empty" => return Ok(Value::Empty),
		"true" => return Ok(Value::Bool (true)),
		"false" => return Ok(Value::Bool (false)),
		"\"" => return parse_multiline_string(lines, line_i),
		_ => {}
	}
	let first_char = value.chars().next().unwrap(); // safety: value cannot be empty because it has to have non-whitespace char(s)
	if first_char.is_ascii_digit() {
		if let Ok(i64_value) = value.parse::<i64>() {return Ok(Value::I64 (i64_value));}
		if let Ok(f64_value) = value.parse::<f64>() {return Ok(Value::F64 (f64_value));}
	}
	if first_char == '"' {
		let last_char = value.chars().last().unwrap(); // safety: value is already assumed to have a first char, therefore it also has a last char
		if last_char != '"' {return Err(ParseEntryError::new(*line_i, "Invalid string, no ending quote found. If this is a single-line string, no characters are allowed after the final quotation mark. If this is meant to be a multi-line string, no characters are allowed after the first quotation mark."))}
		return Ok(Value::String (value[1 .. value.len()-1].to_string()));
	}
	
	Err(ParseEntryError::new(*line_i, "Invalid value, must be 'empty', 'true', 'false', a valid integer, a valid decimal number, a string enclosed in quotes, or a multiline quote starting with a single '\"' character."))
}



fn parse_multiline_string(lines: &[&str], line_i: &mut usize) -> Result<Value, ParseEntryError> {
	let mut output = String::new();
	let start_i = *line_i;
	*line_i += 1;
	let mut curr_line = lines[*line_i].trim_start();
	while curr_line.starts_with('"') {
		output += &curr_line[1..];
		output.push('\n');
		*line_i += 1;
		if *line_i == lines.len() {break;}
		curr_line = lines[*line_i].trim_start();
	}
	*line_i -= 1;
	output.pop();
	if *line_i == start_i {
		return Err(ParseEntryError::new(start_i, String::from("Invalid value, multiline strings cannot be empty")));
	}
	Ok(Value::String (output))
}
