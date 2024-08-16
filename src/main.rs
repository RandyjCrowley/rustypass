use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::{env, fs, process};
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use cocoon::Cocoon;
use dotenv::dotenv;
use rpassword::read_password;
use sha3::{Sha3_512, Digest};
use hex;

fn main() {
    let password = init();

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
        "create" => create_password_workflow(&password),
        "delete" => delete_password_workflow(&password),
        "search" => search_password_workflow(),
        _ => display_help_workflow()
    }

}

fn init() -> String {
    dotenv().ok();
    init_login()
}

fn init_login() -> String {
    let mut int = 0;
    loop {
        if int == 3 {
            println!("Too many password attempts. Exiting...");
            process::exit(0);
        }
        println!("Please enter your password:");
        // Read the password without displaying it
        let password = read_password().expect("Failed to read password");

        let expected_hash = env::var("PASSWORD_HASH").expect("PASSWORD_HASH not set");

        // Create a Sha3-512 hasher instance
        let mut hasher = Sha3_512::new();

        // Write input data
        hasher.update(&password);

        // Read the hash digest and convert it to hex
        let result = hasher.finalize();
        let hex_string = hex::encode(result);

        // Compare the generated hash string to the expected hash
        if hex_string == expected_hash {
            print!("\x1b[A\x1b[2K");
            print!("\x1b[A\x1b[2K");
            print!("\x1b[A\x1b[2K");
            print!("\x1b[A\x1b[2K");
            return password; // Return the correct password
        } else {
            int += 1;
            print!("\x1b[A\x1b[2K");
            print!("\x1b[A\x1b[2K");
            println!("The hash does not match. Please try again.");
        }
    }
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
fn create_password_workflow(password: &str) {
    // Retrieve the encrypted and decrypted paths from environment variables
    let encrypted_path = env::var("ENCRYPTED_FILE").expect("ENCRYPTED_FILE not set");
    let decrypted_path = env::var("DECRYPTED_FILE").expect("DECRYPTED_FILE not set");

    // Decrypt the file (if it exists)
    if Path::new(&encrypted_path).exists() {
        decrypt_file(password.as_bytes());
    } else {
        // If the decrypted file does not exist, create it with an empty array
        let mut file = File::create(&decrypted_path).expect("Failed to create decrypted file");
        file.write_all(b"[]").expect("Failed to write to decrypted file");
    }

    let mut file_content = String::new();
    let mut file = fs::OpenOptions::new().read(true).open(&decrypted_path).expect("Failed to open decrypted file");
    file.read_to_string(&mut file_content).expect("Failed to read decrypted file");

    // Parse the JSON into a dynamic data structure (Value)
    let mut data: Value = serde_json::from_str(&file_content).unwrap_or_else(|_| Value::Array(vec![]));

    let mut site = String::new();
    let mut username = String::new();
    let mut password_input = String::new();

    // Gather input from the user
    println!("Enter your site:");
    io::stdin().read_line(&mut site).expect("Failed to read site");
    site = site.trim().to_string();

    println!("Enter your username:");
    io::stdin().read_line(&mut username).expect("Failed to read username");
    username = username.trim().to_string();

    println!("Enter your password:");
    io::stdin().read_line(&mut password_input).expect("Failed to read password");
    password_input = password_input.trim().to_string();

    if password_input.is_empty() {
        println!("Generating Password...");
        password_input = "password".parse().unwrap();
    }

    let user_info = Information {
        site,
        username,
        password: password_input,
    };

    // Append the new user to the existing data
    if let Some(array) = data.as_array_mut() {
        array.push(serde_json::to_value(user_info).unwrap());
    } else {
        data = Value::Array(vec![serde_json::to_value(user_info).unwrap()]);
    }

    // Write the updated data back to the decrypted file
    let json_data = serde_json::to_string_pretty(&data).unwrap();
    let mut file = fs::OpenOptions::new().write(true).truncate(true).open(&decrypted_path).expect("Failed to open decrypted file for writing");
    file.write_all(json_data.as_bytes()).expect("Failed to write updated data to decrypted file");

    // Encrypt the updated file
    encrypt_file(password.as_bytes());

    // Delete the decrypted file after encryption
    if Path::new(&decrypted_path).exists() {
        fs::remove_file(&decrypted_path).expect("Failed to delete decrypted file");
    }
}

fn delete_password_workflow(password: &str) {
    todo!("delete_password_workflow")
}

fn display_help_workflow() {
    todo!("display_help_workflow")
}

fn encrypt_file(password: &[u8]) {
    // Retrieve the encrypted and decrypted paths from environment variables
    let output_path = env::var("ENCRYPTED_FILE").expect("ENCRYPTED_FILE not set");
    let input_path = env::var("DECRYPTED_FILE").expect("DECRYPTED_FILE not set");

    let mut input_file = File::open(input_path).expect("uh oh");
    let mut buffer = Vec::new();
    input_file.read_to_end(&mut buffer).expect(" uh oh");

    let mut cocoon = Cocoon::new(password);

    let mut output_file = File::create(output_path).expect("uh oh");
    cocoon.dump(buffer, &mut output_file).expect("uh oh");
}

fn decrypt_file(password: &[u8]) {
    // Retrieve the encrypted and decrypted paths from environment variables
    let input_path = env::var("ENCRYPTED_FILE").expect("ENCRYPTED_FILE not set");
    let output_path = env::var("DECRYPTED_FILE").expect("DECRYPTED_FILE not set");

    // Open the encrypted file
    let mut input_file = File::open(input_path).expect("uh oh");

    // Create a Cocoon instance with the provided password
    let cocoon = Cocoon::new(password);

    // Decrypt the data
    let decrypted_data = cocoon.parse(&mut input_file).expect(" uh oh");

    // Write the decrypted content to the output file
    let mut output_file = File::create(output_path).expect("uh oh ");
    output_file.write_all(&decrypted_data).expect("uh oh ");
}
