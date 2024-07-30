use easy_configuration_format as ecf; // recommended way to import functionality
use std::collections::HashMap;



fn main() {
	
	
	// load settings
	
	let (layout, mut values, errors) = ecf::parse_settings(include_str!("example_settings.txt"));
	
	println!("======== Layout: ========");
	for layout_entry in &layout {println!("{layout_entry:?}");}
	
	println!("\n\n\n======== Values: ========");
	for (key, value) in &values {println!("{key}: {value:?}");}
	
	println!("\n\n\n======== Errors: ========");
	for error in errors {println!("{error:?}");}
	
	
	// update settings
	
	update_settings(&mut values);
	
	
	// alter settings
	
	values.insert(String::from("example key"), ecf::Value::Empty);
	values.insert(String::from("new key"), ecf::Value::String (String::from("new value")));
	
	
	// save settings
	
	let (contents, errors) = ecf::format_settings(layout, &values);
	
	println!("\n\n\n======== New Contents: ========");
	println!("\"\"\"");
	println!("{contents}");
	println!("\"\"\"");
	
	println!("\n\n\n======== Errors: ========");
	for error in errors {println!("{error:?}");}
	
	
}





pub const UPDATER_FUNCTIONS: &[fn(&mut HashMap<String, ecf::Value>)] = &[
	update_1_to_2, // updates from format 1 to format 2
	// etc
];

pub fn update_1_to_2(settings: &mut HashMap<String, ecf::Value>) {
	// this example doesn't actually have a format 2, this is just to give an idea of how this should be done
}



pub fn update_settings(settings: &mut HashMap<String, ecf::Value>) {
	let settings_format = ecf::get_int!("SETTINGS FORMAT", settings, err {println!("Warning: {err}."); return;}) as usize;
	for updater_function in UPDATER_FUNCTIONS.iter().skip(settings_format - 1) {
		updater_function(settings);
	}
}
