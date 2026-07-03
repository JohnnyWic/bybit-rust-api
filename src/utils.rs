use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use log::{debug};

// use regex::Captures;
use regex::Regex;
use std::str::FromStr;
use std::sync::OnceLock;

#[derive(Debug)]
pub enum OptionType {
    Put,
    Call,
}

#[derive(Debug)]
pub struct BybitInfo {
    pub base: String,
    pub expire: DateTime<Utc>,
    pub strike_price: f32,
    pub side: OptionType,
    pub quote: Option<String>,
}

pub fn parse_expiration_date(date: &str) -> DateTime<Utc> {
    let naive_date = NaiveDate::parse_from_str(date, "%d%b%y")
        .expect("error parsing expire date from bybit symbol");

    return naive_date
        .and_hms_opt(8, 0, 0)
        .expect("error creating utc datetime object from bybit symbol")
        .and_utc();
}

pub fn calculate_years_to_maturity(expire: DateTime<Utc>) -> f32 {
    debug!("expire date time obj: {}", expire);
    let time_to_expiration: TimeDelta = expire - Utc::now();
    debug!("time_to_expiration: {}", time_to_expiration);
    let seconds_to_expiration = time_to_expiration.num_seconds();
    debug!("seconds_to_expiration: {}", seconds_to_expiration);


    let years_to_expiration = (seconds_to_expiration)  as f32 / (60 * 60 * 24 * 365) as f32;
    debug!("years_to_expiration: {}", years_to_expiration);
    return years_to_expiration;
}

pub fn extract_bybit_info(symbol: &str) -> Option<BybitInfo> {
    static RE: OnceLock<Regex> = OnceLock::new();

    let re = RE.get_or_init(|| {
        Regex::new(r"(?<base>\w+)-(?<expire>\d+\w+\d+)-(?<strike_price>\d+\.?\d*)-(?<side>C|P)(?:-(?<quote>USDT))?")
            .expect("invalid regex extracting bybit infos from symbol!")
    });

    re.captures(symbol).map(|caps| BybitInfo {
        base: caps["base"].to_string(),
        expire: parse_expiration_date(&caps["expire"]),
        strike_price: f32::from_str(&caps["strike_price"]).unwrap(),
        side: match &caps["side"] {
            "C" => OptionType::Call,
            "P" => OptionType::Put,
            _ => unreachable!(),
        },
        quote: caps.name("quote").map(|m| m.as_str().to_string()),
    })
}

