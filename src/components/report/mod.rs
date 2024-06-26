pub(crate) mod command;

use chrono::prelude::*;

use self::command::Report;
use crate::{config::read_config, routes::power::fetch_power_data};

pub(crate) async fn handle_command(cmd: Report) {
    if let Ok(config) = read_config() {
        if config.hoymiles_token.is_empty() {
            println!("You have to run `hoymiles login` first to set the hoymiles_token in the config file.");
            return;
        }

        let local: DateTime<Local> = Local::now();

        // Format the date as "YYYY-MM-DD"
        let formatted_date = local.format("%Y-%m-%d").to_string();

        let mut tasks = vec![];

        for sid in cmd.sids {
            tasks.push(tokio::spawn(fetch_power_data(
                sid,
                formatted_date.clone(),
                config.hoymiles_token.clone(),
                super::power::command::Resolution::Year,
            )))
        }

        let mut results = vec![];

        for task in tasks {
            results.push(task.await.unwrap());
        }

        let mut combined_records: std::collections::BTreeMap<String, f32> =
            std::collections::BTreeMap::new();

        for result in results {
            match result {
                Ok(records) => {
                    for record in records {
                        if let Some(date) = record.date {
                            let power = record.power;
                            *combined_records.entry(date).or_insert(0.0) += power;
                        }
                    }
                }
                Err(e) => eprintln!("Error fetching data {:?}", e),
            }
        }

        println!("{:#?}", combined_records);
    }
}
