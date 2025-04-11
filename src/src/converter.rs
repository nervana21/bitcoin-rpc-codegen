use std::fs;

use anyhow::Result;
use serde_json::{json, Value};

fn main() -> Result<()> {
    // Read the existing api.json
    let api_json = fs::read_to_string("api.json")?;
    let api: Value = serde_json::from_str(&api_json)?;

    // Create new schema-compliant structure
    let mut new_api = json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "Bitcoin Core RPC API Documentation",
        "description": "Documentation for Bitcoin Core's RPC API commands",
        "type": "object",
        "commands": {}
    });

    // Convert each command
    if let Some(commands) = api["commands"].as_object() {
        let mut new_commands = serde_json::Map::new();

        for (name, command_array) in commands {
            if let Some(command) = command_array.as_array().and_then(|arr| arr.first()) {
                let mut new_command = command.clone();

                // Ensure required fields exist
                if !new_command.as_object().unwrap().contains_key("name") {
                    new_command.as_object_mut().unwrap().insert("name".to_string(), json!(name));
                }

                // Convert arguments to schema format
                if let Some(args) = new_command["arguments"].as_array() {
                    let new_args: Vec<Value> = args
                        .iter()
                        .map(|arg| {
                            let mut new_arg = arg.clone();
                            if !new_arg.as_object().unwrap().contains_key("names") {
                                new_arg
                                    .as_object_mut()
                                    .unwrap()
                                    .insert("names".to_string(), json!([]));
                            }
                            new_arg
                        })
                        .collect();
                    new_command
                        .as_object_mut()
                        .unwrap()
                        .insert("arguments".to_string(), json!(new_args));
                }

                // Convert results to schema format
                if let Some(results) = new_command["results"].as_array() {
                    let new_results: Vec<Value> = results
                        .iter()
                        .map(|result| {
                            let mut new_result = result.clone();
                            if !new_result.as_object().unwrap().contains_key("inner") {
                                new_result
                                    .as_object_mut()
                                    .unwrap()
                                    .insert("inner".to_string(), json!([]));
                            }
                            new_result
                        })
                        .collect();
                    new_command
                        .as_object_mut()
                        .unwrap()
                        .insert("results".to_string(), json!(new_results));
                }

                new_commands.insert(name.clone(), json!([new_command]));
            }
        }

        new_api["commands"] = json!(new_commands);
    }

    // Write the converted JSON to a new file
    fs::write("api.schema.json", serde_json::to_string_pretty(&new_api)?)?;

    println!("Successfully converted api.json to schema-compliant format");
    Ok(())
}
