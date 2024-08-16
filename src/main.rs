use std::io;

mod auth;
mod file_ops;
mod workflows;
mod models;


fn main() {
    let master_password = auth::initialize_application();

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
        "search" => workflows::search_password_workflow(),
        _ => workflows::display_help_workflow(),
    }
}
