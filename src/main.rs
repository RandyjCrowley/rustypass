use std::{io, process};

mod auth;
mod file_ops;
mod workflows;
mod models;

fn main() {
    if let Ok(master_password) = auth::initialize_application() {
        loop {
            handle_page_change();
            println!("What would you like to do? (create, update, delete, search, help, quit)");

            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Failed to read line");

            let user_input = user_input.trim().to_ascii_lowercase();

            match user_input.as_str() {
                "create" => {
                    if let Err(e) = workflows::create_password_workflow(&master_password) {
                        eprintln!("Error creating password: {}", e);
                    }
                },
                "delete" => {
                    if let Err(e) = workflows::delete_password_workflow(&master_password) {
                        eprintln!("Error deleting password: {}", e);
                    }
                },
                "search" => {
                    if let Err(e) = workflows::search_password_workflow(&master_password) {
                        eprintln!("Error searching password: {}", e);
                    }
                },
                "update" => {
                    if let Err(e) = workflows::search_password_workflow(&master_password) {
                        eprintln!("Error searching password: {}", e);
                    }
                }
                "help" => workflows::display_help_workflow(),
                "quit" => break,
                _ => println!("Invalid command. Type 'help' for available commands."),
            }
        }
    } else {
        println!("Failed to initialize application. Exiting.");
        process::exit(1);
    }

    println!("Thank you for using Rusty Password Manager. Goodbye!");
}

fn handle_page_change() {
    print!("\x1B[2J\x1B[1;1H");
    println!("######################");
    println!("Rusty Password Manager");
    println!("######################\n");
}