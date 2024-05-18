use prost::Message;
use serde_json::json;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub mod grid {
    pub mod power {
        include!(concat!(env!("OUT_DIR"), "/power.rs"));
    }
}

impl Display for grid::power::GridPowerResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(pd) = self.pd.clone() {
            // Print the table header
            writeln!(f, "{:<8} | {:>10}", "Time", "Power")?;
            writeln!(f, "{:-<8}-+-{:->10}", "", "")?;

            // Iterate over the times and power readings and print them in a table
            for (time, power) in self.time.iter().zip(pd.power.iter()) {
                writeln!(f, "{:<8} | {:>10.2}", time, power)?;
            }
        } else {
            writeln!(f, "No power data available.")?;
        }

        Ok(())
    }
}

#[allow(unused)]
#[derive(Deserialize, Debug)]
pub(crate) struct HoyMilesResponse {
    pub(crate) status: String,
    pub(crate) message: String,
    pub(crate) data: String,
}

#[derive(Debug, Serialize)]
pub(crate) enum Error {
    VerifyToken(String),
    FetchPowerData(String),
    DecodePowerData(String),
}

pub(crate) async fn fetch_power_data(ssid: u32, date: String, token: String) -> Result<(), Error> {
    let client = reqwest::Client::new();

    let url = "https://neapi.hoymiles.com/pvm-data/api/0/station/data/count_power_by_day";

    let body = json!({
        "sid": ssid,
        "date": date
    });

    let response = client
        .post(url)
        .header("authorization", token)
        .json(&body)
        .send()
        .await
        .map_err(|e| Error::FetchPowerData(e.to_string()))?;

    if response.status().is_success() {
        let bytes = response.bytes().await.unwrap();
        if let Ok(hoymiles_response) = serde_json::from_slice::<HoyMilesResponse>(&bytes) {
            println!("{:#?}", hoymiles_response);
            if hoymiles_response.status == *"100" {
                Err(Error::VerifyToken(hoymiles_response.message))
            } else {
                Err(Error::DecodePowerData(("Undefined error").to_string()))
            }
        } else {
            match grid::power::GridPowerResponse::decode(bytes) {
                Ok(decoded) => {
                    println!("{}", decoded);
                    Ok(())
                }
                Err(e) => Err(Error::DecodePowerData(format!(
                    "Failed to decode response: {}",
                    e
                ))),
            }
        }
    } else {
        Err(Error::FetchPowerData(format!(
            "Cannot fetch power data: {}",
            response.status()
        )))
    }
}
