use prost::Message;
use serde_json::json;

use serde::{Deserialize, Serialize};

pub mod grid {
    pub mod power {
        include!(concat!(env!("OUT_DIR"), "/power.rs"));
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

#[derive(Debug, Serialize)]
pub(crate) struct PowerRecord {
    pub(crate) time: String,
    pub(crate) power: f32,
}

pub(crate) async fn fetch_power_data(
    ssid: u32,
    date: String,
    token: String,
) -> Result<Vec<PowerRecord>, Error> {
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

    let mut records: Vec<PowerRecord> = Vec::new();

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
                    if let Some(pd) = decoded.pd {
                        for (time, power) in decoded.time.iter().zip(pd.power.iter()) {
                            records.push(PowerRecord {
                                time: time.clone(),
                                power: *power,
                            });
                        }
                    }

                    Ok(records)
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
