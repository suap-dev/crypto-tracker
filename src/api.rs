use std::time::Instant;
use serde::Deserialize;

use crate::error::CryoError;

pub const MIN_API: &str = "https://min-api.CRYPTOCOMPARE.com";
pub const GET_BTC_EUR: &str = "/data/v2/histominute?fsym=BTC&tsym=EUR&limit=100";

pub async fn get_data() -> Result<String, CryoError> {
    let now = Instant::now();

    let response = reqwest::get(format!("{}{}", MIN_API, GET_BTC_EUR))
        .await
        .map_err(|e| CryoError::FetchError(format!("{:?}", e)))?;

    let raw_data = response
        .text()
        .await
        .map_err(|e| CryoError::ParseError(format!("{:?}", e)))?;

    let parsed: ApiResponse =
        serde_json::from_str(&raw_data).map_err(|e| CryoError::ParseError(format!("{:?}", e)))?;

    // println!("{:?}", parsed);
    for record in parsed.Data.Data {
        print!("{} ", record.open);
    }
    println!();

    let elapsed = now.elapsed();

    Ok(format!(
        "Got response with {} characters in {} miliseconds",
        raw_data.len(),
        elapsed.as_millis()
    ))
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    Data: Data,
}
#[derive(Debug, Deserialize)]
pub struct Data {
    TimeFrom: u64,
    TimeTo: u64,
    Data: Vec<PriceData>,
}

#[derive(Debug, Deserialize)]
pub struct PriceData {
    time: u64,
    high: f64,
    low: f64,
    open: f64,
    close: f64,
    volumefrom: f64,
    volumeto: f64,
}
