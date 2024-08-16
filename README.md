
**Rusty Password Manager**
==========================

A simple command-line password manager written in Rust.

**Features**
------------

* Stores passwords securely using Cocoon encryption
* Allows users to create, delete, and search for passwords
* Generates passwords if none is provided
* Uses environment variables to store encrypted and decrypted files

**Usage**
-----

1. Clone the repository: `git clone https://github.com/RandyjCrowley/rustypass.git`
2. Run `cargo run` to build and run the program
3. Follow the prompts to create, delete, or search for passwords

**Environment Variables**
------------------------

* `ENCRYPTED_FILE`: The path to the encrypted file (default: `encrypted.cocoon`)
* `DECRYPTED_FILE`: The path to the decrypted file (default: `decrypted.json`)

**Security Notes**
-----------------

* Make sure to store your environment variables securely
* Use a secure password for encryption
* Do not hardcode sensitive information in the code

**Contributing**
--------------

Feel free to contribute by submitting pull requests or issues.