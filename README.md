# Rust Git Commit Script

This is a simple Rust script that prompts the user to enter a commit message, adds all changes using `git add .`, and then commits the changes using `git commit -m`.

## Prerequisites

- Rust: Make sure you have Rust installed on your system. You can install Rust by following the instructions on the [official Rust website](https://www.rust-lang.org/).

- Git: Ensure that Git is installed on your system. You can download Git from the [official Git website](https://git-scm.com/).

## Usage

1. Clone this repository to your local machine:

   ```bash
   git clone https://github.com/EsinShadrach/gitgo-rust
   cd gitgo-rust
   ```

2. Build the Rust executable:

   ```bash
   cargo build --release
   ```

3. Run the script:

   ```bash
   ./target/release/rust-git-commit-script
   ```

4. Enter a commit message when prompted. Press Enter to commit, and repeat the process for multiple commits.

## Notes

- The script automatically adds all changes using `git add .` before committing.

- If there are any errors during the script execution, appropriate error messages will be displayed.
