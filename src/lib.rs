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
//! <br>
//! <br>
//! <br>
//! 
//! A settings file is intended to be represented in code using two main values: the layout vec and the values hashmap. The layout vec describes the layout of the settings file according to how it was when it was parsed, and modifying it at runtime isn't recommended (because there should no need to do so). The values hashmap simply stores the key-value (String, ecf::Value) pairs, and this is what your code will interact with.
//! 
//! Also, I strongly recommend using an automatic format upgrading system like what's shown in the example.
//! 
//! <br>
//! <br>



pub mod data;
pub use data::*;



use std::collections::{HashMap, HashSet};





pub fn parse_str(contents: impl AsRef<str>) -> (Vec<LayoutEntry>, HashMap<String, Value>, Vec<ParseEntryError>) {
	let mut layout = vec!();
	let mut values = HashMap::new();
	let mut errors = vec!();
	
	let lines = contents.as_ref().split('\n').collect::<Vec<_>>();
	let mut line_i = 0;
	loop {
		let result = parse_line(&lines, &mut line_i, &mut layout, &mut values);
		if let Err(err) = result {
			layout.push(LayoutEntry::Comment (lines[line_i].to_string()));
			errors.push(err);
		}
		line_i += 1;
		if line_i >= lines.len() {break;}
	}
	
	(layout, values, errors)
}



pub fn parse_line(
	lines: &[&str],
	line_i: &mut usize,
	layout: &mut Vec<LayoutEntry>,
	values: &mut HashMap<String, Value>,
) -> Result<(), ParseEntryError> {
	
	let line = lines[*line_i].trim();
	if line.is_empty() {
		layout.push(LayoutEntry::Empty);
		return Ok(());
	}
	
	if line == "##" {
		layout.push(parse_multiline_comment(lines, line_i)?);
		return Ok(());
	}
	if line.starts_with("#") {
		layout.push(LayoutEntry::Comment (line[1..].to_string()));
		return Ok(());
	}
	
	let colon_index = line.find(':');
	let Some(colon_index) = colon_index else {return Err(ParseEntryError::new(*line_i, "No colon (':') was found, either add a colon after the key or mark this as a comment."));};
	if colon_index == 0 {return Err(ParseEntryError::new(*line_i, "Lines cannot start with a colon."));}
	let key = &line[..colon_index];
	if values.contains_key(key) {return Err(ParseEntryError::new(*line_i, format!("Key \"{key}\" is already defined.")));}
	let value = parse_value(lines, line_i, colon_index)?;
	layout.push(LayoutEntry::Key (key.to_string()));
	values.insert(key.to_string(), value);
	
	Ok(())
}



pub fn parse_multiline_comment(
	lines: &[&str],
	line_i: &mut usize,
) -> Result<LayoutEntry, ParseEntryError> {
	
	let start_line_i = *line_i;
	let mut output = String::new();
	*line_i += 1;
	while lines[*line_i] != "##" {
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



pub fn parse_value(lines: &[&str], line_i: &mut usize, colon_index: usize) -> Result<Value, ParseEntryError> {
	let line = lines[*line_i].trim();
	
	// find start of value
	let value_start_i =
		line.char_indices()
		.skip(colon_index + 1)
		.find(|(_i, c)| !c.is_whitespace());
	let Some((value_start_i, _c)) = value_start_i else {return Err(ParseEntryError::new(*line_i, "No value was found for this key (if this is meant to be empty, please set the value as 'empty')."));};
	
	let value = &line[value_start_i..];
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



pub fn parse_multiline_string(lines: &[&str], line_i: &mut usize) -> Result<Value, ParseEntryError> {
	let start_line_i = *line_i;
	let mut output = String::new();
	*line_i += 1;
	while lines[*line_i] != "\"" {
		output += lines[*line_i];
		output.push('\n');
		*line_i += 1;
		if *line_i == lines.len() {
			*line_i = start_line_i;
			return Err(ParseEntryError::new(start_line_i, "Could not find an end of this multiline string. To end a multiline string, the last line should be nothing but a quotation mark (\")."));
		}
	}
	output.pop();
	Ok(Value::String (output))
}





pub fn format_data(layout: impl AsRef<[LayoutEntry]>, values: &HashMap<String, Value>) -> (String, Vec<FormatEntryError>) {
	let layout = layout.as_ref();
	let mut output = String::new();
	if layout.is_empty() {return (output, vec!());}
	let mut errors = vec!();
	let mut printed_keys = HashSet::new();
	for entry in layout {
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
				let value = values.get(key);
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
	for (key, value) in values {
		if printed_keys.contains(key) {continue;}
		output += key;
		output += ": ";
		output += &value.format();
		output.push('\n');
	}
	output.pop();
	(output, errors)
}
