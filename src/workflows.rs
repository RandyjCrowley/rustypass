use crate::file_ops::{decrypt_file, read_decrypted_data, write_decrypted_data, encrypt_file, create_empty_decrypted_file};
use crate::models::Information;
use std::{env, fs, io, process};
use std::path::Path;
use serde_json::Value;
use crate::auth;

pub fn create_password_workflow(master_password: &str) {
    let encrypted_path = env::var("ENCRYPTED_FILE").expect("ENCRYPTED_FILE not set");
    let decrypted_path = env::var("DECRYPTED_FILE").expect("DECRYPTED_FILE not set");

    if Path::new(&encrypted_path).exists() {
        decrypt_file(master_password.as_bytes());
    } else {
        create_empty_decrypted_file(&decrypted_path);
    }

    let mut data = read_decrypted_data(&decrypted_path);
    let new_entry = collect_user_information();

    append_new_entry(&mut data, new_entry);
    write_decrypted_data(&decrypted_path, &data);
    encrypt_and_cleanup(master_password.as_bytes(), &decrypted_path);

    auth::clear_previous_lines(100);
}

fn collect_user_information() -> Information {
    let mut site = String::new();
    let mut username = String::new();
    let mut password_input = String::new();

    println!("Enter your site:");
    io::stdin().read_line(&mut site).expect("Failed to read site");
    println!("Enter your username:");
    io::stdin().read_line(&mut username).expect("Failed to read username");
    println!("Enter your password:");
    io::stdin().read_line(&mut password_input).expect("Failed to read password");

    let site = site.trim().to_string();
    let username = username.trim().to_string();
    let password_input = if password_input.trim().is_empty() {
        println!("Generating password...");
        "password".to_string()
    } else {
        password_input.trim().to_string()
    };

    Information {
        site,
        username,
        password: password_input,
    }
}

fn append_new_entry(data: &mut Value, new_entry: Information) {
    if let Some(array) = data.as_array_mut() {
        array.push(serde_json::to_value(new_entry).expect("Failed to convert new entry to JSON"));
    } else {
        *data = Value::Array(vec![serde_json::to_value(new_entry).expect("Failed to convert new entry to JSON")]);
    }
}

pub fn delete_password_workflow(_password: &str) {
    todo!("Implement delete_password_workflow");
}

pub fn search_password_workflow(password: &str) {
    loop {
        let mut search = String::new();
        println!("What would you like to search by (site, username, password):");
        io::stdin().read_line(&mut search).expect("Failed to read search input");

        let user_input = search.trim().to_ascii_lowercase();

        if user_input == "site" || user_input == "username" || user_input == "password" {
            let mut keyword = String::new();
            println!("What {} would you like to search for?", user_input);
            io::stdin().read_line(&mut keyword).expect("Failed to read search keyword");

            let encrypted_path = env::var("ENCRYPTED_FILE").expect("ENCRYPTED_FILE not set");
            let decrypted_path = env::var("DECRYPTED_FILE").expect("DECRYPTED_FILE not set");

            if Path::new(&encrypted_path).exists() {
                decrypt_file(password.as_bytes());
            } else {
                create_empty_decrypted_file(&decrypted_path);
            }

            let data = read_decrypted_data(&decrypted_path);
            handle_search(data, keyword.trim(), user_input.as_str());
        } else if user_input == "quit" {
            process::exit(1);
        } else if user_input == "back" {
            todo!("Implement logic to go back to the main menu");
        } else {
            auth::clear_previous_lines(4);
        }
    }
}

fn handle_search(json_data: Value, user_value: &str, user_key: &str) {
    print!("\x1B[2J\x1B[1;1H");
    if let Value::Array(arr) = json_data {
        for item in arr {
            if let Value::Object(ref map) = item {
                if let Some(value) = map.get(user_key) {
                    if value == &Value::String(user_value.to_string()) {
                        println!("Match found:");
                        println!("  Site: {}", item["site"]);
                        println!("  Username: {}", item["username"]);
                        println!("  Password: {}", item["password"]);
                        println!();
                    }
                }
            }
        }
    } else {
        println!("Expected a JSON array at the top level.");
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

fn encrypt_and_cleanup(password: &[u8], decrypted_path: &str) {
    encrypt_file(password);
    if Path::new(decrypted_path).exists() {
        fs::remove_file(decrypted_path).expect("Failed to delete decrypted file");
    }
}
