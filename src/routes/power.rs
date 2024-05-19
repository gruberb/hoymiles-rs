use prost::Message;
use serde_json::json;

use super::get_url;
use crate::components::power::command::Resolution;
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
    pub(crate) date: String,
    pub(crate) time: String,
    pub(crate) power: f32,
}

pub(crate) async fn fetch_power_data(
    sid: u32,
    date: String,
    token: String,
    resolution: Resolution,
) -> Result<Vec<PowerRecord>, Error> {
    let client = reqwest::Client::new();

    let url = get_url(resolution.clone());

    let body = json!({
        "sid": sid,
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
            if resolution == Resolution::Week {
                match grid::power::WeeklyGridPowerResponse::decode(bytes) {
                    Ok(decoded) => {
                        for daily_response in decoded.daily_responses {
                            for (time, power) in daily_response
                                .time
                                .iter()
                                .zip(daily_response.pd.unwrap().power.iter())
                            {
                                records.push(PowerRecord {
                                    date: daily_response.date.clone(),
                                    time: time.clone(),
                                    power: *power,
                                });
                            }
                        }

                        return Ok(records);
                    }
                    Err(e) => {
                        return Err(Error::DecodePowerData(format!(
                            "Failed to decode response: {}",
                            e
                        )))
                    }
                }
            }
            match grid::power::GridPowerResponse::decode(bytes) {
                Ok(decoded) => {
                    if let Some(pd) = decoded.pd {
                        for (time, power) in decoded.time.iter().zip(pd.power.iter()) {
                            records.push(PowerRecord {
                                date: decoded.date.clone(),
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
            response.text().await.unwrap()
        )))
    }
}
