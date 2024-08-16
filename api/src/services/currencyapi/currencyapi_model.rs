use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct LatestExchangeRate {
    pub code: String,
    pub value: f64,
}

pub type LatestData = HashMap<String, LatestExchangeRate>;

#[derive(Debug, Deserialize, Clone)]
pub struct LatestMeta {
    pub last_updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LatestResult {
    pub meta: LatestMeta,
    pub data: LatestData,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DurationRound, TimeDelta};

    #[test]
    fn test_deserialize() {
        let json = r#"
            {
                "meta": {
                    "last_updated_at": "2024-07-13T23:59:59Z"
                },
                "data": {
                    "USD": {
                        "code": "USD",
                        "value": 1.0
                    },
                    "EUR": {
                        "code": "EUR",
                        "value": 1.0
                    }
                }
            }
        "#;

        let actual = serde_json::from_str::<LatestResult>(json).unwrap();
        actual
            .meta
            .last_updated_at
            .duration_trunc(TimeDelta::try_days(1).unwrap())
            .unwrap();
    }
}
