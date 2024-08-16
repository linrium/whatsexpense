use crate::api::report::constants::ReportError;
use crate::api::report::ExpenseByRangeEntity;
use crate::api::transaction::TransactionEntity;
use async_trait::async_trait;
use bson::oid::ObjectId;
use bson::{doc, from_document};
use futures::StreamExt;
use mongodb::Collection;
use std::sync::Arc;

#[async_trait]
pub trait ReportRepoExt: Send + Sync {
    async fn get_expenses_by_range(
        &self,
        user_id: ObjectId,
        from: chrono::DateTime<chrono::Utc>,
        to: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<ExpenseByRangeEntity>, ReportError>;
}

pub type ReportRepoDyn = Arc<dyn ReportRepoExt + Send + Sync>;

pub struct ReportRepo {
    pub transaction_col: Collection<TransactionEntity>,
}

#[async_trait]
impl ReportRepoExt for ReportRepo {
    async fn get_expenses_by_range(
        &self,
        user_id: ObjectId,
        from: chrono::DateTime<chrono::Utc>,
        to: chrono::DateTime<chrono::Utc>,
    ) -> Result<Vec<ExpenseByRangeEntity>, ReportError> {
        let mut cursor = self
            .transaction_col
            .aggregate(vec![
                doc! {
                    "$match": doc! {
                        "$and": [
                            doc! {
                                "issuedAt": doc! {
                                    "$gte": from,
                                },
                            },
                            doc! {
                                "issuedAt": doc! {
                                    "$lt": to,
                                },
                            },
                            doc! {
                                "userId": user_id,
                            },
                        ]
                    }
                },
                doc! {
                    "$project": doc! {
                        "issuedAt": doc! {
                            "$dateToString": doc! {
                                "format": "%Y-%m-%d",
                                "date": "$issuedAt"
                            },
                        },
                        "userId": 1,
                        "amount": 1,
                        "type": 1,
                        "categoryId": 1
                    }
                },
                doc! {
                    "$group": doc! {
                        "_id": doc! {
                            "categoryId": "$categoryId",
                            "issuedAt": "$issuedAt"
                        },
                        "amount": doc! {
                            "$sum": "$amount"
                        }
                    }
                },
                doc! {
                    "$project": doc! {
                        "_id": 0,
                        "category_id": "$_id.categoryId",
                        "issued_at": "$_id.issuedAt",
                        "amount": "$amount"
                    }
                },
                doc! {
                    "$sort": doc! {
                        "issued_at": 1
                    }
                },
            ])
            .await
            .map_err(|e| ReportError::Unknown(e.into()))?;

        let mut expenses = vec![];
        while let Some(Ok(document)) = cursor.next().await {
            let expense: ExpenseByRangeEntity =
                from_document(document).map_err(|e| ReportError::Unknown(e.into()))?;
            expenses.push(expense);
        }

        Ok(expenses)
    }
}
