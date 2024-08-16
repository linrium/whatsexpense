use async_trait::async_trait;
use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;

use crate::api::category::Category;
use crate::api::infer::models::*;
use crate::api::infer::tools::{
    make_infer_category_tool, make_infer_transaction_tool, CategoryToolRaw, TransactionToolRaw,
};
use crate::api::infer::{InferOptions, InferServiceExt};
use crate::common::errors::AppError;
use crate::services::llm::LLMServiceDyn;

pub struct TextInferService {
    pub llm_service: LLMServiceDyn,
}

impl TextInferService {
    const DEFAULT_CURRENCY: &'static str = "USD";

    async fn infer_categories(
        &self,
        prompt: String,
        categories: Vec<Category>,
    ) -> Result<Vec<CategoryTool>, AppError> {
        let fn_obj = make_infer_category_tool(categories);
        let (contents, _) = self.llm_service.chat_with_fn(prompt, fn_obj).await.unwrap();

        let args = contents
            .into_iter()
            .map(|content| serde_json::from_str::<CategoryToolRaw>(&content).unwrap())
            .map(|content| content.into())
            .collect::<Vec<CategoryTool>>();

        Ok(args)
    }

    async fn infer_transactions(
        &self,
        prompt: String,
        currencies: Vec<String>,
    ) -> Result<(InvoiceTool, String), AppError> {
        let fn_obj = make_infer_transaction_tool(currencies.clone());
        let (contents, completion) = self.llm_service.chat_with_fn(prompt, fn_obj).await?;

        let transaction_tools = contents
            .into_iter()
            .map(|content| serde_json::from_str::<TransactionToolRaw>(&content))
            .flat_map(|content| content.map_err(|e| AppError::Unknown(e.into())))
            .map(|content| content.into())
            .collect::<Vec<TransactionTool>>();

        let default_currency = currencies
            .first()
            .cloned()
            .unwrap_or(Self::DEFAULT_CURRENCY.to_string())
            .to_string();
        let (currency, issued_at) = transaction_tools
            .first()
            .cloned()
            .map(|tx| (tx.currency, tx.issued_at))
            .unwrap_or((default_currency, chrono::Utc::now()));

        let total = transaction_tools.iter().map(|tx| tx.amount).sum::<f64>();

        let invoice_tool = InvoiceTool {
            transactions: transaction_tools,
            currency,
            total,
            issued_at,
            subtotal: Some(total),
            card_number: None,
            discounts: vec![],
            taxes: vec![],
        };

        Ok((invoice_tool, completion))
    }
}

#[async_trait]
impl InferServiceExt for TextInferService {
    async fn infer(
        &self,
        prompt: String,
        options: InferOptions,
    ) -> Result<(InvoiceTool, String), AppError> {
        let ((invoice_tool, completion), category_tools) = tokio::try_join!(
            self.infer_transactions(prompt.clone(), options.currencies),
            self.infer_categories(prompt, options.categories)
        )?;

        let transaction_tools = invoice_tool.transactions;
        let transaction_tools = transaction_tools
            .into_iter()
            .zip_longest(category_tools.into_iter())
            .map(|pair| match pair {
                Left(tx) => tx,
                Right(category) => TransactionTool {
                    category_id: category.category_id,
                    r#type: category.r#type,
                    ..Default::default()
                },
                Both(tx, category) => TransactionTool {
                    title: tx.title,
                    currency: tx.currency,
                    amount: tx.amount,
                    quantity: tx.quantity,
                    unit: tx.unit,
                    issued_at: tx.issued_at,
                    category_id: category.category_id,
                    r#type: category.r#type,
                },
            })
            .collect::<Vec<TransactionTool>>();

        let invoice_tool = InvoiceTool {
            transactions: transaction_tools,
            ..invoice_tool
        };

        Ok((invoice_tool, completion))
    }
}
