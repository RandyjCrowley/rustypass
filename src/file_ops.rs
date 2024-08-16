use std::{env, fs};
use std::fs::File;
use std::io::{Read, Write};
use serde_json::Value;
use cocoon::Cocoon;

pub fn encrypt_file(password: &[u8]) {
    let output_path = env::var("ENCRYPTED_FILE").expect("ENCRYPTED_FILE not set");
    let input_path = env::var("DECRYPTED_FILE").expect("DECRYPTED_FILE not set");

    let mut input_file = File::open(&input_path).expect("Failed to open decrypted file for encryption");
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer).expect("Failed to read decrypted file for encryption");

    let mut cocoon = Cocoon::new(password);
    let mut output_file = File::create(&output_path).expect("Failed to create encrypted file");
    cocoon.dump(buffer, &mut output_file).expect("Failed to encrypt file");
}

pub fn decrypt_file(password: &[u8]) {
    let input_path = env::var("ENCRYPTED_FILE").expect("ENCRYPTED_FILE not set");
    let output_path = env::var("DECRYPTED_FILE").expect("DECRYPTED_FILE not set");

    let mut input_file = File::open(&input_path).expect("Failed to open encrypted file");
    let cocoon = Cocoon::new(password);
    let decrypted_data = cocoon.parse(&mut input_file).expect("Failed to decrypt file");

    let mut output_file = File::create(&output_path).expect("Failed to create decrypted file");
    output_file.write_all(&decrypted_data).expect("Failed to write decrypted data to file");
}

pub fn read_decrypted_data(decrypted_path: &str) -> Value {
    let mut file_content = String::new();
    let mut file = fs::OpenOptions::new()
        .read(true)
        .open(decrypted_path)
        .expect("Failed to open decrypted file");
    file.read_to_string(&mut file_content).expect("Failed to read decrypted file");
    serde_json::from_str(&file_content).unwrap_or_else(|_| Value::Array(vec![]))
}

pub fn write_decrypted_data(decrypted_path: &str, data: &Value) {
    let json_data = serde_json::to_string_pretty(data).expect("Failed to serialize data to JSON");
    let mut file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(decrypted_path)
        .expect("Failed to open decrypted file for writing");
    file.write_all(json_data.as_bytes()).expect("Failed to write updated data to decrypted file");
}

pub fn create_empty_decrypted_file(decrypted_path: &str) {
    let mut file = File::create(decrypted_path).expect("Failed to create decrypted file");
    file.write_all(b"[]").expect("Failed to write to decrypted file");
}
