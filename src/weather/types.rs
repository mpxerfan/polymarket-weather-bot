use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherForecast {
    pub location: String,
    pub temperature: f64,
    pub precipitation_chance: f64,
    pub condition: String,
    pub timestamp: DateTime<Utc>,
    pub forecast_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct NOAAResponse {
    pub properties: NOAAProperties,
}

#[derive(Debug, Deserialize)]
pub struct NOAAProperties {
    pub periods: Vec<NOAAPeriod>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NOAAPeriod {
    pub name: String,
    pub temperature: i32,
    pub temperature_unit: String,
    pub detailed_forecast: String,
    pub short_forecast: String,
}
