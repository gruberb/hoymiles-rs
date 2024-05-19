pub(crate) mod command;

use self::command::{FileType, Power};
use crate::{
    config::read_config,
    routes::power::{fetch_power_data, PowerRecord},
};

pub(crate) async fn handle_command(cmd: Power) {
    match read_config() {
        Ok(config) => {
            if config.hoymiles_token.is_empty() {
                println!(
                    "You have to run `hoymiles login` first to set the hoymiles_token in the config file."
                );
                return;
            }
            let power_data =
                fetch_power_data(cmd.sid, cmd.date, config.hoymiles_token, cmd.resolution).await;

            if power_data.is_err() {
                println!(
                    "Failed to fetch power data: {:#?}",
                    power_data.err().unwrap()
                );
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

fn print_power_records_table(records: &[PowerRecord]) {
    // Print the table header
    println!("{:<10} | {:<8} | {:>10}", "Date", "Time", "Power");
    println!("{:-<10}-+-{:-<8}-+-{:->10}", "", "", "");

    // Iterate over the records and print them in a table format
    for record in records {
        println!(
            "{:<10} | {:<8} | {:>10.2}",
            record.date, record.time, record.power
        );
    }
}
