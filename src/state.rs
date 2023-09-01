use std::fs;
use std::fs::File;
use std::io::Read;

use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut data = String::new();
    data = fs::read_to_string(file_name).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();
    let state: Map<String, Value> = json.as_object().unwrap().clone();
    state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name.to_string(), new_data.to_string()).expect("Unable to write file");
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_afile() {
        let data = read_file("./info.json");
        println!("{:?}", data);
    }
}