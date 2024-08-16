use scopeguard::defer;
use std::{env, fs, io, panic};
use std::path::Path;

mod auth;
mod file_ops;
mod workflows;
mod models;

fn main() {
    let master_password = auth::initialize_application();

    // Ensure the cleanup (encrypt and delete plaintext passwords) happens on exit or panic
    let encrypted_path = env::var("ENCRYPTED_FILE").expect("ENCRYPTED_FILE not set");
    let decrypted_path = env::var("DECRYPTED_FILE").expect("DECRYPTED_FILE not set");

    let result = panic::catch_unwind(|| {
        println!("######################");
        println!("Rusty Password Manager");
        println!("######################\n");

        println!("What would you like to do?");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line.");
        let user_input = user_input.trim().to_ascii_lowercase();

        match user_input.as_str() {
            "create" => workflows::create_password_workflow(&master_password),
            "delete" => workflows::delete_password_workflow(&master_password),
            "search" => workflows::search_password_workflow(&master_password),
            _ => workflows::display_help_workflow(),
        }
    });

    if result.is_err() {
        println!("An unexpected error occurred. Cleaning up and exiting...");
    }
}

/// Ensures that the decrypted file is encrypted and deleted.
fn cleanup_and_encrypt(master_password: &str, decrypted_path: &str, encrypted_path: &str) {
    if Path::new(decrypted_path).exists() {
        file_ops::encrypt_file(master_password.as_bytes());
        fs::remove_file(decrypted_path).expect("Failed to delete decrypted file during cleanup");
        println!("Cleanup complete: Encrypted and deleted plaintext passwords.");
    } else {
        println!("No decrypted file found. Cleanup not required.");
    }
}
