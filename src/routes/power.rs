use bytes::Bytes;
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
    pub(crate) date: Option<String>,
    pub(crate) time: Option<String>,
    pub(crate) day: Option<String>,
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

    let records: Vec<PowerRecord> = Vec::new();

    if response.status().is_success() {
        let bytes = response.bytes().await.unwrap();
        if let Ok(hoymiles_response) = serde_json::from_slice::<HoyMilesResponse>(&bytes) {
            if hoymiles_response.status == *"100" {
                Err(Error::VerifyToken(hoymiles_response.message))
            } else {
                Err(Error::DecodePowerData(("Undefined error").to_string()))
            }
        } else {
            match resolution {
                Resolution::Day => return decode_daily_power_data(bytes, records),
                Resolution::Week => return decode_weekly_power_data(bytes, records),
                Resolution::Month => return decode_monthly_power_data(bytes, records),
                Resolution::Year => return decode_yearly_power_data(bytes, records),
            }
        }
    } else {
        Err(Error::FetchPowerData(format!(
            "Failed to fetch power data: {}",
            response.status()
        )))
    }
}

fn decode_daily_power_data(
    bytes: Bytes,
    mut records: Vec<PowerRecord>,
) -> Result<Vec<PowerRecord>, Error> {
    match grid::power::GridPowerResponse::decode(bytes) {
        Ok(decoded) => {
            if let Some(pd) = decoded.pd {
                for (time, power) in decoded.time.iter().zip(pd.power.iter()) {
                    records.push(PowerRecord {
                        date: Some(decoded.date.clone()),
                        time: Some(time.clone()),
                        day: None,
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

fn decode_weekly_power_data(
    bytes: Bytes,
    mut records: Vec<PowerRecord>,
) -> Result<Vec<PowerRecord>, Error> {
    let decoded = grid::power::WeeklyGridPowerResponse::decode(bytes);
    match decoded {
        Ok(decoded) => {
            for daily_response in decoded.daily_responses {
                for (time, power) in daily_response
                    .time
                    .iter()
                    .zip(daily_response.pd.unwrap().power.iter())
                {
                    records.push(PowerRecord {
                        date: Some(daily_response.date.clone()),
                        time: Some(time.clone()),
                        day: None,
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

fn decode_monthly_power_data(
    bytes: Bytes,
    mut records: Vec<PowerRecord>,
) -> Result<Vec<PowerRecord>, Error> {
    match grid::power::MonthGridPowerResponse::decode(bytes) {
        Ok(decoded) => {
            if let Some(pd) = decoded.pd {
                for (day, power) in decoded.day.iter().zip(pd.power.iter()) {
                    records.push(PowerRecord {
                        date: None,
                        time: None,
                        day: Some(day.clone()),
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

fn decode_yearly_power_data(
    bytes: Bytes,
    mut records: Vec<PowerRecord>,
) -> Result<Vec<PowerRecord>, Error> {
    match grid::power::YearGridPowerResponse::decode(bytes) {
        Ok(decoded) => {
            if let Some(pd) = decoded.pd {
                for (date, power) in decoded.date.iter().zip(pd.power.iter()) {
                    records.push(PowerRecord {
                        date: Some(date.clone()),
                        time: None,
                        day: None,
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
