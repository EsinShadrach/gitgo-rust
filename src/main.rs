use std::{
    io::Write,
    process::{exit, Command},
};

fn main() {
    loop {
        let mut commit_message = String::new();
        print!("Enter commit message: ");
        std::io::stdout().flush().unwrap(); // Flush the buffer to ensure the prompt is displayed immediately.
        std::io::stdin()
            .read_line(&mut commit_message)
            .expect("Failed To Read Line");
        if commit_message.ends_with("\n") {
            commit_message.pop();
            create_commit(&commit_message);
        } else {
            break;
        }
    }
}

fn create_commit(commit_message: &str) {
    match Command::new("git").arg("add").arg(".").spawn() {
        Ok(mut status) => {
            if status
                .wait()
                .expect("Failed to wait for 'git add' process")
                .success()
            {
                match Command::new("git")
                    .arg("commit")
                    .arg("-m")
                    .arg(commit_message)
                    .spawn()
                {
                    Ok(mut commit_status) => {
                        if !commit_status
                            .wait()
                            .expect("Failed to wait for 'git commit' process")
                            .success()
                        {
                            eprintln!("Failed to commit changes");
                            exit(1);
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to spawn 'git commit': {}", e);
                        exit(1);
                    }
                }
            } else {
                eprintln!("Failed to add changes");
                exit(1);
            }
        }
        Err(e) => {
            eprintln!("Failed to spawn 'git add': {}", e);
            exit(1);
        }
    }
}
