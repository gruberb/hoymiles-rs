use serde::Deserialize;
use serde_json::json;

use super::LOGIN_URL;

#[allow(unused)]
#[derive(Debug, Deserialize)]
pub(crate) struct LoginResponse {
    pub(crate) status: String,
    pub(crate) message: String,
    pub(crate) data: Option<LoginData>,
    pub(crate) system_notice: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum LoginData {
    Error { user_name: String },
    Success { token: String },
}

pub(crate) async fn login(
    user_name: String,
    password: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let digest = md5::compute(password);
    let hex_string = format!("{:x}", digest);
    let password: &str = &hex_string;

    let login_response = client
        .post(LOGIN_URL)
        .json(&json!({
            "user_name": user_name,
            "password": password,
        }))
        .send()
        .await?;

    let l = login_response.text().await?;
    let l = l.as_str();
    let login: LoginResponse = serde_json::from_str(l)?;

    match login.data {
        Some(LoginData::Success { token }) => Ok(token),
        Some(LoginData::Error { user_name }) => {
            eprintln!("Login Error: {}", user_name);
            Err("Login Error".into())
        }
        None => {
            eprintln!("Error: No data in response");
            Err("Error: No data in response".into())
        }
    }
}
