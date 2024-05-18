pub(crate) mod command;

use self::command::Power;
use crate::routes::power::fetch_power_data;

pub(crate) async fn handle_command(cmd: Power) {
    load_env();

    match std::env::var("HOYMILES_TOKEN") {
        Ok(token) => {
            if token.is_empty() {
                println!(
                    "You have to run `hoymiles login` first to set the HOYMILES_TOKEN env variable"
                );
                return;
            }
            let _ = fetch_power_data(cmd.ssid, cmd.date, token).await;
        }
        Err(_) => println!(
            "You have to run `hoymiles loign` first to set the HOYMILES_TOKEN env variable"
        ),
    }
}

fn load_env() {
    dotenv::dotenv().ok();
}
