use easy_configuration_format as ecf; // recommended way to import functionality
use std::collections::HashMap;



fn main() {
	
	
	
	// load settings
	
	pub struct UpdaterFunctionArg {}
	pub const UPDATER_FUNCTIONS: &[fn(&mut HashMap<String, ecf::Value>, &mut UpdaterFunctionArg)] = &[
		update_1_to_2, // updates from format 1 to format 2
		// etc
	]; // because there's 1 updater function, the crate will know that the current format version is 2
	
	pub fn update_1_to_2(_settings: &mut HashMap<String, ecf::Value>, _args: &mut UpdaterFunctionArg) {
		println!("this example doesn't actually have a format 2, this is just to show how updates would be done");
	}
	
	let mut update_args = UpdaterFunctionArg {};
	let (mut ecf_file, _did_run_updaters, errors) = ecf::File::from_str(include_str!("example_settings.ecf"), UPDATER_FUNCTIONS, &mut update_args);
	
	ecf_file.add_missing_values([
		("This key must exist, and the default is I64(64)", ecf::Value::I64(64)),
	].into_iter());
	
	
	
	// print file data
	println!("======== Layout: ========");
	for layout_entry in &ecf_file.layout {println!("{layout_entry:?}");}
	
	println!("\n\n\n======== Values: ========");
	for (key, value) in &ecf_file.values {println!("{key}: {value:?}");}
	
	println!("\n\n\n======== Errors: ========");
	for error in errors {println!("{error:?}");}
	
	
	
	// get and alter settings
	
	println!("\n\n\n======== Editing Values: ========");
	
	let example_value_str = ecf_file.get_str("example key").clone();
	println!("value in 'example key' as a str: {example_value_str:?}");
	let example_value_int = ecf_file.get_int("example key").clone();
	println!("value in 'example key' as an int: {example_value_int:?}"); // this prints an `Err` variant
	
	ecf_file.insert(String::from("example key"), ecf::Value::Empty);
	ecf_file.insert(String::from("new key"), ecf::Value::String (String::from("new value")));
	
	
	
	// save settings
	
	let (contents, errors) = ecf_file.to_str();
	
	println!("\n\n\n======== New Contents: ========");
	println!("\"\"\"");
	println!("{contents}");
	println!("\"\"\"");
	
	println!("\n\n\n======== Errors: ========");
	for error in errors {println!("{error:?}");}
	
	
	
}
