use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use bson::oid::ObjectId;

use crate::api::report::report_repo::ReportRepoDyn;
use crate::api::report::ExpenseByRange;
use crate::common::errors::AppError;

#[async_trait]
pub trait ReportServiceExt: Send + Sync {
    async fn get_expenses_by_range(
        &self,
        user_id: ObjectId,
        start: chrono::DateTime<chrono::Utc>,
        end: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<ExpenseByRange>, AppError>;
}

#[allow(dead_code)]
pub type ReportServiceDyn = Arc<dyn ReportServiceExt + Send + Sync>;

pub struct ReportService {
    pub repo: ReportRepoDyn,
}

#[async_trait]
impl ReportServiceExt for ReportService {
    async fn get_expenses_by_range(
        &self,
        user_id: ObjectId,
        from: chrono::DateTime<chrono::Utc>,
        to: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<ExpenseByRange>, AppError> {
        let expenses: Vec<ExpenseByRange> = self
            .repo
            .get_expenses_by_range(user_id, from, to)
            .await
            .map(|v| v.into_iter().map(Into::into).collect())
            .map_err(|e| AppError::from(e))?;

        let mut filled_expenses = vec![];
        let mut current = from;

        let mut expense_map = HashMap::new();
        for expense in expenses {
            expense_map
                .entry(expense.issued_at.clone())
                .or_insert_with(Vec::new)
                .push(expense)
        }

        while current < to {
            let key = current.format("%Y-%m-%d").to_string();
            if let Some(expenses) = expense_map.remove(&key) {
                filled_expenses.extend(expenses)
            } else {
                filled_expenses.push(ExpenseByRange {
                    issued_at: key,
                    category_id: "unknown".to_string(),
                    amount: 0.0,
                })
            }

            current = current + chrono::Duration::days(1);
        }

        Ok(filled_expenses)
    }
}
