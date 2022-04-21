use std::fs;
use std::fs::File;
use std::io::Read;

use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

pub fn read_file(file_name: String) -> Map<String, Value> {
	let mut file = File::open(file_name).expect("File not found");
	let mut contents = String::new();
	file
		.read_to_string(&mut contents)
		.expect("Could not read file");
	let json: Value = serde_json::from_str(&contents).expect("Could not parse json");
	let state: Map<String, Value> = json.as_object().unwrap().clone();
	state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
	let json = json!(state);
	let json_string = json.to_string();
	fs::write(file_name, json_string).expect("Could not write to file");
}
