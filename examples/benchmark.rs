// to run: `cargo run --release --example benchmark`

use easy_configuration_format as ecf;
use toml::Table;
use std::{hint::black_box, time::{Duration, Instant}};



fn main() {
	
	debug_assert!(false, "only run this in release mode!");
	
	const WARMUP_COUNT: usize = 16;
	const RUN_COUNT: usize = 1024;
	
	for _ in 0..WARMUP_COUNT {
		do_ecf_benchmark();
		do_toml_benchmark();
	}
	let mut total_ecf_duration = Duration::from_secs(0);
	let mut total_toml_duration = Duration::from_secs(0);
	for _ in 0..RUN_COUNT {
		total_ecf_duration += do_ecf_benchmark();
		total_toml_duration += do_toml_benchmark();
	}
	let average_ecf_duration = total_ecf_duration / RUN_COUNT as u32;
	let average_toml_duration = total_toml_duration / RUN_COUNT as u32;
	
	println!("Average ecf time:  {average_ecf_duration:?}");
	println!("Average toml time: {average_toml_duration:?}");
	
}



pub fn do_ecf_benchmark() -> Duration {
	let start = Instant::now();
	
	// parse settings
	let (mut ecf_file, _did_run_updaters, _errors) = ecf::File::from_str(include_str!("example_settings.ecf"), &[], &mut ());
	
	// query and edit settings
	ecf_file.add_missing_values([
		("This key must exist, and the default (if missing) is Value::I64(64)", ecf::Value::I64(64)),
	].into_iter());
	let _example_value = black_box(ecf_file.get_str("example key"));
	let _example_blank = black_box(ecf_file.get_empty("example blank"));
	let _example_int = black_box(ecf_file.get_int("example int"));
	ecf_file.insert(String::from("example key"), ecf::empty());
	ecf_file.insert(String::from("new key"), ecf::string("new value"));
	
	// format settings
	let (_formatted, _errors) = black_box(ecf_file.to_str());
	
	start.elapsed()
}



pub fn do_toml_benchmark() -> Duration {
	let start = Instant::now();
	
	// parse settings
	let mut settings = include_str!("example_settings.toml").parse::<Table>().unwrap();
	
	// query and edit settings
	for kv in &[
		("This key must exist, and the default (if missing) is Value::I64(64)", toml::Value::Integer(64))
	] {
		let entry = settings.entry(kv.0);
		entry.or_insert(kv.1.clone());
	}
	let _example_value = black_box(settings.get("example key"));
	let _example_blank = black_box(settings.get("example blank"));
	let _example_int = black_box(settings.get("example int"));
	settings.insert(String::from("example_key"), toml::Value::String(String::from("null")));
	settings.insert(String::from("new_key"), toml::Value::String(String::from("new value")));
	
	// format settings
	let _formatted = black_box(toml::to_string(&settings).unwrap());
	
	start.elapsed()
}
