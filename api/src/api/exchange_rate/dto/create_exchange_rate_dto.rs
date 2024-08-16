pub struct CreateExchangeRateData {
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
    pub code: String,
    pub value: f64,
}

pub struct CreateExchangeRateInput {
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
    pub code: String,
    pub value: f64,
}
