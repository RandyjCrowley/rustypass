use std::io;

fn main() {
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

fn search_password_workflow() {
    todo!("search_password_workflow")
}

fn create_password_workflow() {
    todo!("create_password_workflow")
}

fn delete_password_workflow() {
    todo!("delete_password_workflow")
}

fn display_help_workflow() {
    todo!("display_help_workflow")
}
