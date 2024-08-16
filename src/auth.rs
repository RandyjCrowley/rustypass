use std::{env, process};
use sha3::{Sha3_512, Digest};
use rpassword::read_password;
use hex;
use dotenv::dotenv;

pub fn initialize_application() -> String {
    dotenv().ok();
    prompt_for_master_password()
}

pub fn prompt_for_master_password() -> String {
    const MAX_ATTEMPTS: usize = 3;
    let mut attempts = 0;

    loop {
        if attempts >= MAX_ATTEMPTS {
            println!("Too many password attempts. Exiting...");
            process::exit(1);
        }

        println!("Please enter your master password:");
        let password = read_password().expect("Failed to read password");

        if verify_password_hash(&password) {
            clear_previous_lines(4);
            return password;
        } else {
            attempts += 1;
            clear_previous_lines(2);
            println!("Password does not match. Please try again.");
        }
    }
}

fn verify_password_hash(password: &str) -> bool {
    let stored_hash = env::var("PASSWORD_HASH").expect("PASSWORD_HASH not set");
    let mut hasher = Sha3_512::new();
    hasher.update(password);
    let computed_hash = hex::encode(hasher.finalize());

    computed_hash == stored_hash
}

pub(crate) fn clear_previous_lines(line_count: usize) {
    for _ in 0..line_count {
        print!("\x1b[A\x1b[2K");
    }
}

