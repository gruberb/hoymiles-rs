use std::io::BufRead;
use std::io::Write;

use self::command::Login;

pub(crate) mod command;

use crate::routes::login::login;

pub async fn handle_command(cmd: Login) {
    match login(cmd.user_name, cmd.password).await {
        Ok(token) => {
            // Update the .env file with the new token
            if let Err(e) = update_env_file("HOYMILES_TOKEN", &token) {
                eprintln!("Failed to update .env file: {}", e);
            } else {
                println!("Set HOYMILES_TOKEN env variable successfully.");
            }
        }
        Err(e) => eprintln!("Error: {:#?}", e),
    }
}

fn update_env_file(key: &str, value: &str) -> std::io::Result<()> {
    let env_file_path = std::path::Path::new(".env");
    let mut lines = vec![];

    if env_file_path.exists() {
        let file = std::fs::File::open(env_file_path)?;
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            if line.starts_with(&format!("{}=", key)) {
                lines.push(format!("{}={}", key, value));
            } else {
                lines.push(line);
            }
        }
    } else {
        lines.push(format!("{}={}", key, value));
    }

    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(env_file_path)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
