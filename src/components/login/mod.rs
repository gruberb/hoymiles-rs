use self::command::Login;

pub(crate) mod command;

use crate::config::{get_config_file, write_config, Config};
use crate::routes::login::login;

pub async fn handle_command(cmd: Login) {
    match login(cmd.user_name, cmd.password).await {
        Ok(token) => {
            let config = Config {
                hoymiles_token: token,
            };
            match write_config(&config) {
                Ok(_) => {
                    println!(
                        "Successfully saved auth token to config file in {}",
                        get_config_file().display()
                    );
                }
                Err(e) => eprintln!("Error: {:#?}", e),
            }
        }
        Err(e) => eprintln!("Error: {:#?}", e),
    }
}
