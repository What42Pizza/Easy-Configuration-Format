use crate::Value;
use std::collections::HashMap;



/// Returns either the value as an str or a short error message
pub fn get_str(setting_name: impl AsRef<str>, settings: &HashMap<String, Value>) -> Result<&str, String> {
	get_value_as_type::<String>(setting_name, settings).map(|v| &**v)
}

/// Returns either the value as an string or a short error message
pub fn get_string_mut(setting_name: impl AsRef<str>, settings: &mut HashMap<String, Value>) -> Result<&mut String, String> {
	get_value_as_type_mut(setting_name, settings)
}



/// Returns either the value as an int or a short error message
pub fn get_int(setting_name: impl AsRef<str>, settings: &HashMap<String, Value>) -> Result<i64, String> {
	get_value_as_type(setting_name, settings).copied()
}

/// Returns either the value as an int or a short error message
pub fn get_int_mut(setting_name: impl AsRef<str>, settings: &mut HashMap<String, Value>) -> Result<&mut i64, String> {
	get_value_as_type_mut(setting_name, settings)
}



/// Returns either the value as an float or a short error message
pub fn get_float(setting_name: impl AsRef<str>, settings: &HashMap<String, Value>) -> Result<f64, String> {
	get_value_as_type(setting_name, settings).copied()
}

/// Returns either the value as an float or a short error message
pub fn get_float_mut(setting_name: impl AsRef<str>, settings: &mut HashMap<String, Value>) -> Result<&mut f64, String> {
	get_value_as_type_mut(setting_name, settings)
}



/// Returns either the value as an bool or a short error message
pub fn get_bool(setting_name: impl AsRef<str>, settings: &HashMap<String, Value>) -> Result<bool, String> {
	get_value_as_type(setting_name, settings).copied()
}

/// Returns either the value as an bool or a short error message
pub fn get_bool_mut(setting_name: impl AsRef<str>, settings: &mut HashMap<String, Value>) -> Result<&mut bool, String> {
	get_value_as_type_mut(setting_name, settings)
}





macro_rules! create_get_macro {
	($get_name:ident, $($type_name_singular:ident)+) => {
		#[macro_export]
		/// Returns the value as an int or runs a block of code with `err` containing the error
		macro_rules! $get_name {
			($setting_name:expr, $settings:expr, $err_name:ident $err_block:block) => {
				match easy_configuration_format::$get_name($setting_name, $settings) {
					Ok(v) => v,
					Err($err_name) => $err_block
				}
			};
		}
	};
}

create_get_macro!(get_str, a String);
create_get_macro!(get_string_mut, a String);
create_get_macro!(get_int, an Int);
create_get_macro!(get_int_mut, an Int);
create_get_macro!(get_float, a Float);
create_get_macro!(get_float_mut, a Float);
create_get_macro!(get_bool, a Bool);
create_get_macro!(get_bool_mut, a Bool);





/// Generic function for getting a setting value as a specific type
pub fn get_value_as_type<T: FromEcfValue>(setting_name: impl AsRef<str>, settings: &HashMap<String, Value>) -> Result<&T, String> {
	let setting_name = setting_name.as_ref();
	let Some(value) = settings.get(setting_name) else {
		return Err(format!("could not find setting \"{setting_name}\""));
	};
	let Some(output) = T::from_ecf_value(value) else {
		return Err(format!("setting \"{setting_name}\" needs to be {}, but it currently is {}", T::TYPE_NAME_SINGULAR, value.type_as_singular_string()));
	};
	Ok(output)
}

/// Mutable version of get_value_as_type()
pub fn get_value_as_type_mut<T: FromEcfValueMut>(setting_name: impl AsRef<str>, settings: &mut HashMap<String, Value>) -> Result<&mut T, String> {
	let setting_name = setting_name.as_ref();
	let Some(value) = settings.get_mut(setting_name) else {
		return Err(format!("could not find setting \"{setting_name}\""));
	};
	let value_type_name_singular = value.type_as_singular_string(); // theoretically this shouldn't be needed here, but for some reason it isn't allowed in the else block
	let Some(output) = T::from_ecf_value_mut(value) else {
		return Err(format!("setting \"{setting_name}\" needs to be {}, but it currently is {}", T::TYPE_NAME_SINGULAR, value_type_name_singular));
	};
	Ok(output)
}



/// Converts an ecf::Value into a generic type
pub trait FromEcfValue: Sized {
	/// Name of the generic type, used for error messages, needs to be singular and uppercase (example: "a String")
	const TYPE_NAME_SINGULAR: &'static str;
	/// Main functionality
	fn from_ecf_value(input: &Value) -> Option<&Self>;
}

/// Mutable version of FromEcfValue
pub trait FromEcfValueMut: FromEcfValue {
	/// Mutable version of FromEcfValue::from_ecf_value()
	fn from_ecf_value_mut(input: &mut Value) -> Option<&mut Self>;
}



impl FromEcfValue for String {
	const TYPE_NAME_SINGULAR: &'static str = "a String";
	fn from_ecf_value(input: &Value) -> Option<&Self> {
		if let Value::String (input_string) = input {
			Some(input_string)
		} else {
			None
		}
	}
}

impl FromEcfValue for i64 {
	const TYPE_NAME_SINGULAR: &'static str = "an Int";
	fn from_ecf_value(input: &Value) -> Option<&Self> {
		if let Value::I64 (input_i64) = input {
			Some(input_i64)
		} else {
			None
		}
	}
}

impl FromEcfValue for f64 {
	const TYPE_NAME_SINGULAR: &'static str = "a Float";
	fn from_ecf_value(input: &Value) -> Option<&Self> {
		if let Value::F64 (input_f64) = input {
			Some(input_f64)
		} else {
			None
		}
	}
}

impl FromEcfValue for bool {
	const TYPE_NAME_SINGULAR: &'static str = "a Bool";
	fn from_ecf_value(input: &Value) -> Option<&Self> {
		if let Value::Bool (input_bool) = input {
			Some(input_bool)
		} else {
			None
		}
	}
}



impl FromEcfValueMut for String {
	fn from_ecf_value_mut(input: &mut Value) -> Option<&mut Self> {
		if let Value::String (input_string) = input {
			Some(input_string)
		} else {
			None
		}
	}
}

impl FromEcfValueMut for i64 {
	fn from_ecf_value_mut(input: &mut Value) -> Option<&mut Self> {
		if let Value::I64 (input_i64) = input {
			Some(input_i64)
		} else {
			None
		}
	}
}

impl FromEcfValueMut for f64 {
	fn from_ecf_value_mut(input: &mut Value) -> Option<&mut Self> {
		if let Value::F64 (input_f64) = input {
			Some(input_f64)
		} else {
			None
		}
	}
}

impl FromEcfValueMut for bool {
	fn from_ecf_value_mut(input: &mut Value) -> Option<&mut Self> {
		if let Value::Bool (input_bool) = input {
			Some(input_bool)
		} else {
			None
		}
	}
}
