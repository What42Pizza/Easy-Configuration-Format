use easy_configuration_format as ecf; // recommended way to import functionality
use std::collections::HashMap;



pub const UPDATER_FUNCTIONS: &[fn(&mut HashMap<String, ecf::Value>, &())] = &[
	update_1_to_2, // updates from format 1 to format 2
	// etc
];

pub fn update_1_to_2(settings: &mut HashMap<String, ecf::Value>, args: &()) {
	println!("this example doesn't actually have a format 2, this is just to give an idea of how this should be done");
}



fn main() {
	
	
	// load settings
	
	let (mut ecf_file, errors) = ecf::parse_settings(include_str!("example_settings.txt"), UPDATER_FUNCTIONS, &());
	
	println!("======== Layout: ========");
	for layout_entry in &ecf_file.layout {println!("{layout_entry:?}");}
	
	println!("\n\n\n======== Values: ========");
	for (key, value) in &ecf_file.values {println!("{key}: {value:?}");}
	
	println!("\n\n\n======== Errors: ========");
	for error in errors {println!("{error:?}");}
	
	
	// alter settings
	
	ecf_file.insert(String::from("example key"), ecf::Value::Empty);
	ecf_file.insert(String::from("new key"), ecf::Value::String (String::from("new value")));
	
	
	// save settings
	
	let (contents, errors) = ecf::format_settings(&ecf_file);
	
	println!("\n\n\n======== New Contents: ========");
	println!("\"\"\"");
	println!("{contents}");
	println!("\"\"\"");
	
	println!("\n\n\n======== Errors: ========");
	for error in errors {println!("{error:?}");}
	
	
}
