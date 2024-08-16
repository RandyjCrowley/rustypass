use std::error::Error;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::{env, fs};
use std::fs::File;
use std::io::{self, Read, Write};
use cocoon::Cocoon;
use dotenv::dotenv;

#[derive(Serialize, Deserialize, Debug)]
struct InputData {
    data: String,
}

fn main() {
    init();
    println!("######################");
    println!("Rusty Password Manager");
    println!("######################");
    println!(" ");
    println!("What would you like to do?");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let input = input.trim().to_ascii_lowercase();

    match input.as_str() {
        "create" => create_password_workflow(),
        "delete" => delete_password_workflow(),
        "search" => search_password_workflow(),
        _ => display_help_workflow()
    }

}

fn init() {
    dotenv().ok();
}

fn search_password_workflow() {
    todo!("search_password_workflow")
}


#[derive(Serialize, Deserialize, Debug)]
struct Information {
    site: String,
    username: String,
    password: String,
}


fn create_password_workflow() {

    // Retrieve the encrypted and decrypted paths from environment variables
    let encrypted_path = env::var("ENCRYPTED_PATH").expect("ENCRYPTED_PATH not set");
    let decrypted_path = env::var("DECRYPTED_PATH").expect("DECRYPTED_PATH not set");
    // let password = b"super_secret_password";
    // encrypt_file("input.txt", "encrypted.cocoon", password);
    // println!("File encrypted successfully.");
    //
    // let password = b"super_secret_password";
    // decrypt_file("encrypted.cocoon", "decrypted.txt", password).expect("TODO: panic message");
    // println!("File decrypted successfully.");

    // return;
    let mut site = String::new();
    let mut username = String::new();
    let mut password = String::new();


    // Gather input from the user
    println!("Enter your site:");
    io::stdin().read_line(&mut site).expect("UH OH");
    site = site.trim().to_string();

    println!("Enter your username:");
    io::stdin().read_line(&mut username).expect("UH OH");
    username = username.trim().to_string();

    println!("Enter your password:");
    io::stdin().read_line(&mut password).expect("UH OH");
    password = password.trim().to_string();

    if password.is_empty() {
        println!("Generating Password...");
        password = "password".parse().unwrap();
    }

    let user_info = Information {
        site,
        username,
        password,
    };



    let mut file_content = String::new();
    let mut file = fs::OpenOptions::new().read(true).open("user_data.json").expect("uh oh");
    file.read_to_string(&mut file_content).expect("uh oh");

    // Parse the JSON into a dynamic data structure (Value)
    let mut data: Value = serde_json::from_str(&file_content).unwrap_or_else(|_| Value::Array(vec![]));

    // Append the new user to the existing data
    if let Some(array) = data.as_array_mut() {
        array.push(serde_json::to_value(user_info).unwrap());
    } else {
        data = Value::Array(vec![serde_json::to_value(user_info).unwrap()]);
    }


    // Write the updated data back to the file
    let json_data = serde_json::to_string_pretty(&data).unwrap();
    let mut file = fs::OpenOptions::new().write(true).truncate(true).open("user_data.json").expect("uh oh");
    file.write_all(json_data.as_bytes()).expect("uh oh");

    return;
    todo!("create_password_workflow")
}

fn delete_password_workflow() {
    todo!("delete_password_workflow")
}

fn display_help_workflow() {
    todo!("display_help_workflow")
}


fn encrypt_file(password: &[u8]) {

    // Retrieve the encrypted and decrypted paths from environment variables
    let output_path = env::var("ENCRYPTED_PATH").expect("ENCRYPTED_PATH not set");
    let input_path = env::var("DECRYPTED_PATH").expect("DECRYPTED_PATH not set");

    let mut input_file = File::open(input_path).expect("uh oh");
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer).expect(" uh oh");

    let mut cocoon = Cocoon::new(password);

    let mut output_file = File::create(output_path).expect("uh oh");
    cocoon.dump(buffer, &mut output_file).expect("uh oh");

    return
}

fn decrypt_file(password: &[u8]) {
    // Retrieve the encrypted and decrypted paths from environment variables
    let input_path = env::var("ENCRYPTED_PATH").expect("ENCRYPTED_PATH not set");
    let output_path = env::var("DECRYPTED_PATH").expect("DECRYPTED_PATH not set");

    // Open the encrypted file
    let mut input_file = File::open(input_path).expect("uh oh");

    // Create a Cocoon instance with the provided password
    let mut cocoon = Cocoon::new(password);

    // Decrypt the data
    let decrypted_data = cocoon.parse(&mut input_file).expect(" uh oh");

    // Write the decrypted content to the output file
    let mut output_file = File::create(output_path).expect("uh oh ");
    output_file.write_all(&decrypted_data).expect("uh oh ");
}