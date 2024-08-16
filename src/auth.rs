use std::{env, process};
use sha3::{Sha3_512, Digest};
use rpassword::read_password;
use dotenv::dotenv;

pub fn initialize_application() -> Result<String, Box<dyn std::error::Error>> {
    dotenv().ok();
    Ok(prompt_for_master_password()?)
}

fn prompt_for_master_password() -> Result<String, Box<dyn std::error::Error>> {
    const MAX_ATTEMPTS: usize = 3;
    let mut attempts = 0;
    loop {
        if attempts >= MAX_ATTEMPTS {
            println!("Too many password attempts. Exiting...");
            process::exit(1);
        }
        println!("Please enter your master password: ");
        let password = read_password()?;
        println!();

        if verify_password_hash(&password)? {
            print!("\x1B[2J\x1B[1;1H");
            return Ok(password);
        } else {
            attempts += 1;
            println!("Password does not match. Please try again.");
        }
    }
}

fn verify_password_hash(password: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let stored_hash = env::var("PASSWORD_HASH")?;
    let mut hasher = Sha3_512::new();
    hasher.update(password);
    let computed_hash = hex::encode(hasher.finalize());

    Ok(computed_hash == stored_hash)
}

pub(crate) fn clear_previous_lines(line_count: usize) {
    for _ in 0..line_count {
        print!("\x1b[A\x1b[2K");
    }
}