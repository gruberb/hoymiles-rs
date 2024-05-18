pub(crate) mod command;

use self::command::{FileType, Power};
use crate::routes::power::{fetch_power_data, PowerRecord};

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
            let power_data = fetch_power_data(cmd.ssid, cmd.date, token).await;

            if power_data.is_err() {
                println!("Failed to fetch power data");
                std::process::exit(1);
            }

            if let Some(format) = cmd.save {
                match format {
                    FileType::Csv => {
                        let mut wtr = csv::Writer::from_path("grid_power.csv").unwrap();
                        for record in &power_data {
                            wtr.serialize(record).unwrap();
                        }
                        wtr.flush().unwrap();
                        println!("Data saved in grid_power.csv")
                    }
                    FileType::Json => {
                        let json_file = std::fs::File::create("grid_power.json").unwrap();
                        serde_json::to_writer_pretty(json_file, &power_data.unwrap()).unwrap();
                        println!("Data saved in grid_power.json");
                    }
                }
            } else {
                print_power_records_table(&power_data.unwrap());
            }
        }
        Err(_) => println!(
            "You have to run `hoymiles loign` first to set the HOYMILES_TOKEN env variable"
        ),
    }
}

fn load_env() {
    dotenv::dotenv().ok();
}

fn print_power_records_table(records: &[PowerRecord]) {
    // Print the table header
    println!("{:<8} | {:>10}", "Time", "Power");
    println!("{:-<8}-+-{:->10}", "", "");

    // Iterate over the records and print them in a table format
    for record in records {
        println!("{:<8} | {:>10.2}", record.time, record.power);
    }
}
