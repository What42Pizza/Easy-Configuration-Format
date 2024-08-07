//! ## Store, save, and load your data in the most simple way possible.
//! 
//! <br>
//! 
//! ## Example settings file:
//! 
//! ```text
//! example key: "example value"
//!
//! example blank: empty
//! example string: "not empty"
//! example int: 3
//! example float: 3.5
//! example bool: true
//! example multiline: "
//! first line (#0)
//! also, because of how strings are defined, you can have " characters inside a string with
//! no escape codes needed
//! last line (#3)
//! "
//! 
//! example namespace.example key: "example value 2"
//! # "namespaces" are entirely made up, there's no direct support for them and they're just
//! # a recommended way to structure settings
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
//! ```
//! 
//! You can find a slightly more formal specification [here](https://github.com/What42Pizza/Easy-Configuration-Format/blob/main/specification.txt).
//! 
//! <br>
//! <br>
//! <br>
//! 
//! A settings file is intended to be represented in code using two main values: the layout vec and the values hashmap. The layout vec describes the layout of the settings file according to how it was when it was parsed, and modifying it at runtime isn't recommended (because there should no need to do so). The values hashmap simply stores the key-value (String, ecf::Value) pairs, and this is what your code will interact with.
//! 
//! Also, I strongly recommend using an automatic format upgrading system like what's shown in the [example](https://github.com/What42Pizza/Easy-Configuration-Format/blob/main/examples/main.rs).
//! 
//! <br>
//! <br>



#![warn(missing_docs)]



/// All the data types used by this crate
pub mod data;
pub use data::*;
/// Utility functions for easy value management
pub mod utils;
pub use utils::*;



use std::collections::{HashMap, HashSet};





/// Converts a settings file into a layout + values, opposite of `format_settings()`
pub fn parse_settings<T>(contents: impl AsRef<str>, update_fns: &[fn(&mut HashMap<String, Value>, &T)], args: &T) -> (File, Vec<ParseEntryError>) {
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
	
	if let Some(version) = version {
		for update_fn in &update_fns[version - 1 ..] {
			(update_fn)(&mut values, args);
		}
	} else {
		errors.push(ParseEntryError {
			line: 0,
			message: String::from("Could not find version, assuming version is latest"),
		});
	}
	
	(File {
		values,
		layout,
		version: update_fns.len() + 1,
	}, errors)
}



fn get_file_version(first_line: &str) -> Option<usize> {
	if !first_line.starts_with("format ") {return None;}
	let format_value_str = first_line[7..].trim();
	let format_value = format_value_str.parse::<usize>();
	format_value.ok()
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
	if line_trimmed.starts_with("#") {
		layout.push(LayoutEntry::Comment (line_trimmed[1..].to_string()));
		return Ok(());
	}
	
	let colon_index = line_trimmed.find(':');
	let Some(colon_index) = colon_index else {return Err(ParseEntryError::new(*line_i, "No colon (':') was found, either add a colon after the key or mark this as a comment."));};
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
			return Err(ParseEntryError::new(start_line_i, "Could not find an end of this multiline comment. To end a multiline comment, the last line should be nothing but '##'."));
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
	if first_char.is_digit(10) {
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
	while curr_line.chars().next() == Some('"') {
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





/// Converts a layout plus values into a formatted settings file, opposite of `parse_settings()`
pub fn format_settings(file: &File) -> (String, Vec<FormatEntryError>) {
	let mut output = format!("format {}\n", file.version);
	if file.layout.is_empty() {return (output, vec!());}
	let mut errors = vec!();
	let mut printed_keys = HashSet::new();
	for entry in &file.layout {
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
				let value = file.get(key);
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
	for (key, value) in &file.values {
		if printed_keys.contains(key) {continue;}
		output += key;
		output += ": ";
		output += &value.format();
		output.push('\n');
	}
	output.pop();
	(output, errors)
}





/// Automatically merge new setting values with existing setting values
pub fn merge_values(existing_values: &mut HashMap<String, Value>, new_values: &HashMap<String, Value>, merge_options: MergeOptions) {
	match merge_options {
		MergeOptions::UpdateOnly => {
			for (key, value) in new_values {
				if existing_values.contains_key(key) {
					existing_values.insert(key.clone(), value.clone());
				}
			}
		}
		MergeOptions::UpdateAndAdd => {
			for (key, value) in new_values {
				existing_values.insert(key.clone(), value.clone());
			}
		}
		MergeOptions::AddOnly => {
			for (key, value) in new_values {
				if !existing_values.contains_key(key) {
					existing_values.insert(key.clone(), value.clone());
				}
			}
		}
		MergeOptions::FullyReplace => {
			*existing_values = new_values.clone();
		}
	}
}

/// Used with `merge_values()`
pub enum MergeOptions {
	/// Only Update the values that already exist in the hashmap
	UpdateOnly,
	/// Update the values that already exist in the hashmap, and add any new key-value pairs that didn't exist
	UpdateAndAdd,
	/// Only add key-value pairs that didn't exist in the hashmap
	AddOnly,
	/// Simple replace the existing hashmap with the new hashmap
	FullyReplace,
}
