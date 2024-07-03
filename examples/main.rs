use easy_configuration_format as ecf; // recommended way to import functionality



fn main() {
	
	
	
	// load settings
	
	let (layout, mut values, errors) = ecf::parse_str(include_str!("example_settings.txt"));
	
	println!("======== Layout: ========");
	println!("{layout:#?}");
	
	println!("\n\n\n======== Values: ========");
	println!("{values:#?}");
	
	println!("\n\n\n======== Errors: ========");
	println!("{errors:#?}");
	
	
	
	// alter settings
	
	values.insert(String::from("example key"), ecf::Value::Empty);
	values.insert(String::from("new key"), ecf::Value::String (String::from("new value")));
	
	
	
	// save settings
	
	let (contents, errors) = ecf::format_data(layout, &values);
	
	println!("\n\n\n======== New Contents: ========");
	println!("\"\"\"");
	println!("{contents}");
	println!("\"\"\"");
	
	println!("\n\n\n======== Errors: ========");
	println!("{errors:#?}");
	
	
	
}
