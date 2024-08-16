**Rusty Password Manager**
========================

A simple, secure password manager built with Rust.

**Features**

* Store and encrypt passwords for various sites and services
* Generate strong, unique passwords for new accounts
* Search and filter stored passwords by site or username
* Delete sensitive data when no longer needed

**Usage**

1. Initialize the application by running `cargo run` in your terminal.
2. Choose an option:
    * Create: Add a new password entry
    * Delete: Remove a password entry (currently not implemented)
    * Search: Find and display stored passwords
    * Help: Display usage instructions

**Installation**

1. Clone this repository using `git clone https://github.com/RandyjCrowley/rustypass.git`
2. Install Rust and Cargo by following the official installation guide <https://www.rust-lang.org/tools/install>
3. Run `cargo build` to compile the project
4. Run `cargo run` to execute the application

**Security**

* All passwords are encrypted using the SHA-512 algorithm.
* Data is stored in a single JSON file, which can be deleted at any time.

**License**

This software is licensed under the MIT License. See LICENSE.md for details.

**Contributing**

Contributions and feedback are welcome! Please submit issues or pull requests through the GitHub repository.

I hope this helps! Let me know if you need any further assistance.