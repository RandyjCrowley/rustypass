use crate::file_ops::{decrypt_file, read_decrypted_data, write_decrypted_data, encrypt_file, create_empty_decrypted_file};
use crate::models::Information;
use std::{env, fs, io};
use std::path::Path;
use serde_json::Value;

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
        array.push(serde_json::to_value(new_entry).unwrap());
    } else {
        *data = Value::Array(vec![serde_json::to_value(new_entry).unwrap()]);
    }
}

pub fn delete_password_workflow(_password: &str) {
    todo!("Implement delete_password_workflow");
}

pub fn search_password_workflow() {
    todo!("Implement search_password_workflow");
}

pub fn display_help_workflow() {
    todo!("Implement display_help_workflow");
}

fn encrypt_and_cleanup(password: &[u8], decrypted_path: &str) {
    encrypt_file(password);
    if Path::new(decrypted_path).exists() {
        fs::remove_file(decrypted_path).expect("Failed to delete decrypted file");
    }
}

