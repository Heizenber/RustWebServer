mod processes;
mod state;
mod to_do;

use processes::process_input;
use serde_json::value::Value;
use serde_json::{json, Map};
use state::{read_file, write_to_file};
use std::env;
use to_do::structs::traits::create::Create;
use to_do::to_do_factory;
use to_do::ItemTypes;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command: &String = &args[1];
    let title: &String = &args[2];

    let mut state: Map<String, Value> = read_file("./state.json");

    let status: String;

    match &state.get(*&title) {
        Some(result) => {
            status = result.to_string().replace('\"', "");
        }
        None => {
            status = "pending".to_string();
        }
    }

    let item = to_do_factory(&status, title).expect(&status);
    process_input(item, command.to_string(), &state);
}
