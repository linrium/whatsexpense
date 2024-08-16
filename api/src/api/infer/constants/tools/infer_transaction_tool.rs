use async_openai::types::{FunctionObject, FunctionObjectArgs};
use chrono::{Datelike, NaiveDate, NaiveDateTime};
use chronoutil::{shift_months, shift_years};
use regex::Regex;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct TransactionToolRaw {
    pub title: String,
    pub currency: String,
    pub amount: String,
    pub quantity: Option<f64>,
    pub unit: Option<String>,
    pub date: Option<String>,
}

pub fn parse_amount_string(amount: String) -> f64 {
    let re = Regex::new(r"/,\./g").unwrap();
    let amount = re.replace_all(&amount, "");

    if amount.contains("sen") {
        return amount.replace("sen", "").parse::<f64>().unwrap_or(0.0) * 1_000.0;
    }

    if amount.contains("man") {
        return amount.replace("man", "").parse::<f64>().unwrap_or(0.0) * 10_000.0;
    }

    if amount.contains("k") {
        return amount.replace("k", "").parse::<f64>().unwrap_or(0.0) * 1_000.0;
    }

    if amount.contains("m") {
        return amount.replace("m", "").parse::<f64>().unwrap_or(0.0) * 1_000_000.0;
    }

    if amount.contains("tr") {
        return amount.replace("tr", "").parse::<f64>().unwrap_or(0.0) * 1_000_000.0;
    }

    if amount.contains("b") {
        return amount.replace("b", "").parse::<f64>().unwrap_or(0.0) * 1_000_000_000.0;
    }

    amount.parse::<f64>().unwrap_or(0.0)
}

pub fn parse_issued_at_string(
    init: chrono::DateTime<chrono::Utc>,
    date_str: Option<String>,
) -> chrono::DateTime<chrono::Utc> {
    let result = match date_str.clone() {
        Some(date_str) => parse_date_string(init, date_str),
        None => None,
    };

    if let Some(result) = result {
        return result;
    }

    match date_str {
        Some(from_now) => parse_from_now_string(init, from_now).unwrap_or(init),
        None => init,
    }
}

pub fn parse_date_string(
    init: chrono::DateTime<chrono::Utc>,
    date_str: String,
) -> Option<chrono::DateTime<chrono::Utc>> {
    let parts = date_str.split("/").collect::<Vec<&str>>();
    if parts.len() == 2 || parts.len() == 3 {
        let day = parts[0].parse::<u32>().unwrap();
        let month = parts[1].parse::<u32>().unwrap();
        let year = if parts.len() == 3 {
            parts[2].parse::<i32>().unwrap()
        } else {
            init.year()
        };

        let naive_datetime = NaiveDate::from_ymd_opt(year, month, day).unwrap_or(init.date_naive());
        let naive_datetime = NaiveDateTime::new(
            naive_datetime,
            chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
        );
        return Some(chrono::DateTime::from_naive_utc_and_offset(
            naive_datetime,
            chrono::Utc,
        ));
    }

    None
}

pub fn parse_from_now_string(
    init: chrono::DateTime<chrono::Utc>,
    from_now: String,
) -> Option<chrono::DateTime<chrono::Utc>> {
    let parts = from_now.split(" ").collect::<Vec<&str>>();
    if parts.len() == 1 && parts[0] == "yesterday" {
        return Some(init - chrono::Duration::days(1));
    }

    if parts.len() == 2 && parts[0] == "last" {
        let unit = parts[1];
        let result = match unit {
            "week" | "weeks" => init - chrono::Duration::weeks(1),
            "month" | "months" => shift_months(init, -1),
            "year" | "years" => shift_months(init, -12),
            _ => init,
        };

        return Some(result);
    }

    if parts.len() == 3 && parts[2] == "ago" {
        let unit = parts[1];
        let value = parts[0].parse::<i64>().unwrap_or(1);

        let result = match unit {
            "day" | "days" => init - chrono::Duration::days(value),
            "week" | "weeks" => init - chrono::Duration::weeks(value),
            "month" | "months" => shift_months(init, -1 * value as i32),
            "year" | "years" => shift_years(init, -1 * value as i32),
            _ => init,
        };

        return Some(result);
    }

    None
}

pub fn make_infer_transaction_tool(currencies: Vec<String>) -> FunctionObject {
    FunctionObjectArgs::default()
        .name("infer_transactions")
        .description("Infer transactions from a given prompt")
        .parameters(json!({
            "type": "object",
            "required": ["title", "currency", "amount"],
            "properties": {
                "title": {
                    "type": "string",
                    "description": "The title of the transaction in the original language",
                },
                "currency": {
                    "type": "string",
                    "description": "Currency to infer transactions for",
                    "enum": currencies,
                },
                "amount": {
                    "type": "string",
                    "description": "The format can be 0.0a where a is the unit e.g. 1.5k, 2.5m, 3.5tr, 40k, etc."
                },
                "quantity": {
                    "type": "number",
                    "description": "The quantity of the product e.g. 2, 3, etc."
                },
                "unit": {
                    "type": "string",
                    "description": "The unit of the product e.g. kg, m, etc."
                },
                "date": {
                    "type": "string",
                    "examples": ["1 hour ago", "yesterday", "2 days ago", "last week", "3 weeks ago", "last month", "2 months ago", "30/04"],
                    "description": "The timestamp of the transaction in format DD/MM or 1 hour ago, yesterday",
                }
            }
        }))
        .build()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_amount_string() {
        assert_eq!(parse_amount_string("1".to_string()), 1.0);
        assert_eq!(parse_amount_string("1.5".to_string()), 1.5);
        assert_eq!(parse_amount_string("1.5k".to_string()), 1_500.0);
        assert_eq!(parse_amount_string("1.5m".to_string()), 1_500_000.0);
        assert_eq!(parse_amount_string("1.5man".to_string()), 15_000.0);
        assert_eq!(parse_amount_string("1.5sen".to_string()), 1500.0);
        assert_eq!(parse_amount_string("1.5tr".to_string()), 1_500_000.0);
        assert_eq!(parse_amount_string("1.5b".to_string()), 1_500_000_000.0);
        assert_eq!(parse_amount_string("1.5t".to_string()), 0.0);
    }

    #[test]
    fn test_parse_date_string() {
        let now = chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
            .unwrap()
            .to_utc();
        assert_eq!(parse_date_string(now, "01/01".to_string()), Some(now));
        assert_eq!(parse_date_string(now, "50/01".to_string()), Some(now));
        assert_eq!(parse_date_string(now, "01/50".to_string()), Some(now));
        assert_eq!(parse_date_string(now, "01/01/2024".to_string()), Some(now));

        let now = chrono::DateTime::parse_from_rfc3339("2024-07-13T00:00:00Z")
            .unwrap()
            .to_utc();
        assert_eq!(
            parse_date_string(now, "30/04".to_string()),
            Some(
                chrono::DateTime::parse_from_rfc3339("2024-04-30T00:00:00Z")
                    .unwrap()
                    .to_utc()
            )
        );
    }

    #[test]
    fn test_parse_from_now_string() {
        let now = chrono::DateTime::parse_from_rfc3339("2024-02-10T00:00:00Z")
            .unwrap()
            .to_utc();
        assert_eq!(
            parse_from_now_string(now, "yesterday".to_string()),
            Some(
                chrono::DateTime::parse_from_rfc3339("2024-02-09T00:00:00Z")
                    .unwrap()
                    .to_utc()
            )
        );
        assert_eq!(
            parse_from_now_string(now, "last week".to_string()),
            Some(
                chrono::DateTime::parse_from_rfc3339("2024-02-03T00:00:00Z")
                    .unwrap()
                    .to_utc()
            )
        );
        assert_eq!(
            parse_from_now_string(now, "last month".to_string()),
            Some(
                chrono::DateTime::parse_from_rfc3339("2024-01-10T00:00:00Z")
                    .unwrap()
                    .to_utc()
            )
        );
        assert_eq!(
            parse_from_now_string(now, "last year".to_string()),
            Some(
                chrono::DateTime::parse_from_rfc3339("2023-02-10T00:00:00Z")
                    .unwrap()
                    .to_utc()
            )
        );
        assert_eq!(
            parse_from_now_string(now, "2 days ago".to_string()),
            Some(
                chrono::DateTime::parse_from_rfc3339("2024-02-08T00:00:00Z")
                    .unwrap()
                    .to_utc()
            )
        );
        assert_eq!(
            parse_from_now_string(now, "2 weeks ago".to_string()),
            Some(
                chrono::DateTime::parse_from_rfc3339("2024-01-27T00:00:00Z")
                    .unwrap()
                    .to_utc()
            )
        );
        assert_eq!(
            parse_from_now_string(now, "2 months ago".to_string()),
            Some(
                chrono::DateTime::parse_from_rfc3339("2023-12-10T00:00:00Z")
                    .unwrap()
                    .to_utc()
            )
        );
        assert_eq!(
            parse_from_now_string(now, "2 years ago".to_string()),
            Some(
                chrono::DateTime::parse_from_rfc3339("2022-02-10T00:00:00Z")
                    .unwrap()
                    .to_utc()
            )
        );
    }

    #[test]
    fn test_parse_issued_at_string() {
        let now = chrono::DateTime::parse_from_rfc3339("2024-07-13T00:00:00Z")
            .unwrap()
            .to_utc();
        assert_eq!(
            parse_issued_at_string(now, None),
            chrono::DateTime::parse_from_rfc3339("2024-07-13T00:00:00Z")
                .unwrap()
                .to_utc()
        );

        let now = chrono::DateTime::parse_from_rfc3339("2024-07-13T00:00:00Z")
            .unwrap()
            .to_utc();
        assert_eq!(
            parse_issued_at_string(now, Some("30/04".to_string())),
            chrono::DateTime::parse_from_rfc3339("2024-04-30T00:00:00Z")
                .unwrap()
                .to_utc()
        );

        let now = chrono::DateTime::parse_from_rfc3339("2024-07-13T00:00:00Z")
            .unwrap()
            .to_utc();
        assert_eq!(
            parse_issued_at_string(now, Some("yesterday".to_string())),
            chrono::DateTime::parse_from_rfc3339("2024-07-12T00:00:00Z")
                .unwrap()
                .to_utc()
        );
    }
}
