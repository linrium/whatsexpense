use crate::api::category::Category;
use crate::api::infer::models::*;
use crate::api::infer::tools::{
    make_infer_category_tool, make_infer_invoice_tool, CategoryToolRaw, InvoiceToolRaw,
};
use crate::api::infer::{InferOptions, InferServiceExt};
use crate::common::errors::AppError;
use crate::services::llm::LLMServiceDyn;
use anyhow::anyhow;
use async_trait::async_trait;

#[derive(Clone)]
pub struct InvoiceInferService {
    pub llm_service: LLMServiceDyn,
}

impl InvoiceInferService {
    async fn infer_categories(
        &self,
        prompt: String,
        categories: Vec<Category>,
    ) -> Result<Vec<CategoryTool>, AppError> {
        if categories.is_empty() {
            return Ok(vec![]);
        }

        if prompt.is_empty() {
            return Ok(vec![]);
        }

        let fn_obj = make_infer_category_tool(categories);
        let (contents, _) = self.llm_service.chat_with_fn(prompt, fn_obj).await?;

        let args = contents
            .into_iter()
            .map(|content| serde_json::from_str::<CategoryToolRaw>(&content).unwrap())
            .map(|content| content.into())
            .collect::<_>();

        Ok(args)
    }

    async fn infer_invoice(
        &self,
        prompt: String,
        currencies: Vec<String>,
    ) -> Result<(InvoiceTool, String), AppError> {
        let fn_obj = make_infer_invoice_tool(currencies);
        let (contents, completion) = self.llm_service.chat_with_fn(prompt, fn_obj).await?;

        let invoice_tool = contents
            .first()
            .cloned()
            .ok_or(AppError::Unknown(anyhow::anyhow!("no invoice")))?;

        let invoice_tool: InvoiceTool = serde_json::from_str::<InvoiceToolRaw>(&invoice_tool)
            .map_err(|e| AppError::Unknown(e.into()))?
            .into();

        Ok((invoice_tool, completion))
    }
}

#[async_trait]
impl InferServiceExt for InvoiceInferService {
    async fn infer(
        &self,
        prompt: String,
        options: InferOptions,
    ) -> Result<(InvoiceTool, String), AppError> {
        if prompt.is_empty() {
            return Err(AppError::Unknown(anyhow!("empty prompt")));
        }

        let (mut invoice_tool, completion) = self.infer_invoice(prompt, options.currencies).await?;

        if invoice_tool.transactions.is_empty() {
            return Err(AppError::Unknown(anyhow!("no transactions")));
        }

        let tx_title = invoice_tool
            .transactions
            .first()
            .cloned()
            .map(|tx| tx.title)
            .unwrap_or_default();

        let category_tool = self
            .infer_categories(tx_title, options.categories)
            .await?
            .first()
            .cloned()
            .unwrap_or_default();

        for tx in invoice_tool.transactions.iter_mut() {
            let category_tool = category_tool.clone();
            tx.category_id = category_tool.category_id;
            tx.r#type = category_tool.r#type;
        }

        Ok((invoice_tool, completion))
    }
}
