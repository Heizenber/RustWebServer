use std::fs;

use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let data = fs::read_to_string(file_name).unwrap();
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
        let _state = read_file("./state.json");
    }
}
