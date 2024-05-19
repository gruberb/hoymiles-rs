pub(crate) mod login;
pub(crate) mod power;

use crate::components::power::command::Resolution;

pub(crate) const LOGIN_URL: &str =
    "https://global.hoymiles.com/platform/api/gateway/iam/auth_login";
pub(crate) const BASE: &str = "https://neapi.hoymiles.com/pvm-data/api/0/station/data";

const DAY_URL: &str = "/count_power_by_day";
const WEEK_URL: &str = "/count_power_by_week";
const MONTH_URL: &str = "/count_eq_by_day_of_month";
const YEAR_URL: &str = "/count_eq_by_day_of_year";

pub fn get_url(resolution: Resolution) -> String {
    match resolution {
        Resolution::Day => BASE.to_owned() + DAY_URL,
        Resolution::Week => BASE.to_owned() + WEEK_URL,
        Resolution::Month => BASE.to_owned() + MONTH_URL,
        Resolution::Year => BASE.to_owned() + YEAR_URL,
    }
}
