use std::sync::Arc;

use async_trait::async_trait;

use crate::api::category::*;
use crate::common::errors::AppError;

#[async_trait]
pub trait CategoryServiceExt: Send + Sync {
    async fn find(&self) -> Result<Vec<Category>, AppError>;
}

pub type CategoryServiceDyn = Arc<dyn CategoryServiceExt + Send + Sync>;

#[derive(Clone)]
pub struct CategoryService {}

#[async_trait]
impl CategoryServiceExt for CategoryService {
    async fn find(&self) -> Result<Vec<Category>, AppError> {
        Ok(vec![
            Category {
                id: "housing".to_string(),
                name: "Housing".to_string(),
                description: "Rent, mortgage, property tax, etc.".to_string(),
                color: "#fdaaaa".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "household_items".to_string(),
                name: "Household items".to_string(),
                description: "Furniture, appliances, etc.".to_string(),
                color: "#fdaad2".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "childcare".to_string(),
                name: "Childcare".to_string(),
                description: "Daycare, babysitting, etc.".to_string(),
                color: "#f3aafd".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "transportation".to_string(),
                name: "Transportation".to_string(),
                description: "Gas, public transport, etc.".to_string(),
                color: "#cbaafd".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "utilities".to_string(),
                name: "Utilities".to_string(),
                description: "Electricity, water, internet, etc.".to_string(),
                color: "#b0aafd".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "groceries".to_string(),
                name: "Groceries".to_string(),
                description: "Food, drinks, etc.".to_string(),
                color: "#aad5fd".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "dining_out".to_string(),
                name: "Dining out".to_string(),
                description: "Restaurants, cafes, etc.".to_string(),
                color: "#94bffc".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "pets".to_string(),
                name: "Pets".to_string(),
                description: "Food, grooming, vet, etc.".to_string(),
                color: "#7ccefd".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "entertainment".to_string(),
                name: "Entertainment".to_string(),
                description: "Movies, games, events, etc.".to_string(),
                color: "#a4eafd".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "healthcare".to_string(),
                name: "Healthcare".to_string(),
                description: "Doctor, dentist, medicine, etc.".to_string(),
                color: "#66faf8".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "insurance".to_string(),
                name: "Insurance".to_string(),
                description: "Health, car, home, etc.".to_string(),
                color: "#aafdef".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "personal_care".to_string(),
                name: "Personal care".to_string(),
                description: "Gym, beauty, clothing, etc.".to_string(),
                color: "#aafddd".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "debts".to_string(),
                name: "Debts".to_string(),
                description: "Credit card, loan, etc.".to_string(),
                color: "#7bfa8c".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "givings".to_string(),
                name: "Givings".to_string(),
                description: "Charity, gifts, etc.".to_string(),
                color: "#b7f85e".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "shopping".to_string(),
                name: "Shopping".to_string(),
                description: "Clothes, electronics, etc.".to_string(),
                color: "#e2f85e".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "education".to_string(),
                name: "Education".to_string(),
                description: "Tuition, books, etc.".to_string(),
                color: "#fdf6aa".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "travel".to_string(),
                name: "Travel".to_string(),
                description: "Flights, hotels, etc.".to_string(),
                color: "#fddfaa".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "miscellaneous".to_string(),
                name: "Miscellaneous".to_string(),
                description: "Other, etc.".to_string(),
                color: "#fdc3aa".to_string(),
                r#type: "outcome".to_string(),
            },
            Category {
                id: "unknown".to_string(),
                name: "Unknown".to_string(),
                description: "Unknown items".to_string(),
                color: "#a1a1aa".to_string(),
                r#type: "outcome".to_string(),
            },
        ])
    }
}
