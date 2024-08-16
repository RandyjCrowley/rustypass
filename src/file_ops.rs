use std::{env};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde_json::Value;
use cocoon::Cocoon;

pub fn encrypt_file(password: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let output_path = env::var("ENCRYPTED_FILE")?;
    let input_path = env::var("DECRYPTED_FILE")?;

    let mut input_file = File::open(&input_path)?;
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer)?;

    let mut cocoon = Cocoon::new(password);
    let mut output_file = File::create(&output_path).expect("");
    cocoon.dump(buffer, &mut output_file).expect("");

    Ok(())
}

pub fn decrypt_file(password: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = env::var("ENCRYPTED_FILE")?;
    let output_path = env::var("DECRYPTED_FILE")?;

    let mut input_file = File::open(&input_path).expect("");
    let cocoon = Cocoon::new(password);
    let decrypted_data = cocoon.parse(&mut input_file).expect("");

    let mut output_file = File::create(&output_path).expect("");
    output_file.write_all(&decrypted_data).expect("");

    Ok(())
}

pub fn read_decrypted_data(decrypted_path: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let mut file = File::open(decrypted_path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    Ok(serde_json::from_str(&file_content)?)
}

pub fn write_decrypted_data(decrypted_path: &str, data: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = serde_json::to_string_pretty(data)?;
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(decrypted_path)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

pub fn create_empty_decrypted_file(decrypted_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = File::create(decrypted_path)?;
    file.write_all(b"[]")?;
    Ok(())
}