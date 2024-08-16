use crate::file_ops::{decrypt_file, read_decrypted_data, write_decrypted_data, encrypt_file, create_empty_decrypted_file};
use crate::models::Information;
use std::{env, fs, io, process};
use std::path::Path;
use serde_json::Value;
use crate::auth;

pub fn create_password_workflow(master_password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let encrypted_path = env::var("ENCRYPTED_FILE")?;
    let decrypted_path = env::var("DECRYPTED_FILE")?;

    if !Path::new(&encrypted_path).exists() {
        create_empty_decrypted_file(&decrypted_path)?;
    } else {
        decrypt_file(master_password.as_bytes())?;
    }

    let mut data = read_decrypted_data(&decrypted_path)?;
    let new_entry = collect_user_information()?;
    append_new_entry(&mut data, new_entry).expect("TODO: panic message");
    write_decrypted_data(&decrypted_path, &data)?;
    encrypt_and_cleanup(master_password.as_bytes(), &decrypted_path)?;

    auth::clear_previous_lines(100);
    Ok(())
}

fn collect_user_information() -> Result<Information, Box<dyn std::error::Error>> {
    let mut inputs = vec![String::new(), String::new(), String::new()];
    let prompts = ["site", "username", "password"];

    for (i, prompt) in prompts.iter().enumerate() {
        println!("Enter your {}:", prompt);
        io::stdin().read_line(&mut inputs[i])?;
    }

    Ok(Information {
        site: inputs[0].trim().to_string(),
        username: inputs[1].trim().to_string(),
        password: if inputs[2].trim().is_empty() {
            println!("Generating password...");
            "password".to_string()
        } else {
            inputs[2].trim().to_string()
        },
    })
}

fn append_new_entry(data: &mut Value, new_entry: Information) -> Result<(), Box<dyn std::error::Error>> {
    if let Value::Array(ref mut array) = data {
        array.push(serde_json::to_value(new_entry)?);
    } else {
        *data = Value::Array(vec![serde_json::to_value(new_entry)?]);
    }
    Ok(())
}

pub fn delete_password_workflow(_password: &str) -> Result<(), Box<dyn std::error::Error>>{
    todo!("Implement delete_password_workflow");
}

pub fn search_password_workflow(password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let encrypted_path = env::var("ENCRYPTED_FILE")?;
    let decrypted_path = env::var("DECRYPTED_FILE")?;

    loop {
        print!("\x1B[2J\x1B[1;1H");
        let mut search = String::new();
        println!("What would you like to search by (site, username, password):");
        io::stdin().read_line(&mut search)?;

        let user_input = search.trim().to_ascii_lowercase();

        if !["site", "username", "password", "quit", "back"].contains(&user_input.as_str()) {
            auth::clear_previous_lines(4);
            continue;
        }

        if user_input == "quit" {
            process::exit(1);
        } else if user_input == "back" {
            // todo!("Implement logic to go back to the main menu");
            continue;
        }

        let mut keyword = String::new();
        println!("What {} would you like to search for?", user_input);
        io::stdin().read_line(&mut keyword)?;

        if Path::new(&encrypted_path).exists() {
            decrypt_file(password.as_bytes())?;
        } else {
            create_empty_decrypted_file(&decrypted_path)?;
        }

        let data = read_decrypted_data(&decrypted_path)?;
        handle_search(data, keyword.trim(), user_input.as_str());
    }
}

fn handle_search(json_data: Value, user_value: &str, user_key: &str) {
    print!("\x1B[2J\x1B[1;1H");
    if let Value::Array(arr) = json_data {
        for item in arr {
            if let Value::Object(ref map) = item {
                if let Some(value) = map.get(user_key) {
                    if let Value::String(ref val) = value {
                        if val == user_value {
                            println!("Match found:");
                            println!("  Site: {}", item["site"]);
                            println!("  Username: {}", item["username"]);
                            println!("  Password: {}", item["password"]);
                            println!();
                        }
                    }
                }
            }
        }
    }
}

pub(crate) fn display_help_workflow() {
    print!("\x1B[2J\x1B[1;1H");
    println!("Rusty Password Manager Help:");
    println!();
    println!("Usage:");
    println!("  create - Create a new password entry");
    println!("  delete - Delete an existing password entry");
    println!("  search - Search for a specific password entry");
    println!();
    println!("Available commands:");
    println!("  back - Go back to the main menu");
    println!("  quit - Quit the program");
    println!();
    println!("Note:");
    println!("  You will be prompted to enter your master password for certain actions.");
}

fn encrypt_and_cleanup(password: &[u8], decrypted_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    encrypt_file(password)?;
    if Path::new(decrypted_path).exists() {
        fs::remove_file(decrypted_path)?;
    }
    Ok(())
}