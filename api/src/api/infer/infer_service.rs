use crate::api::infer::models::{InferMode, InvoiceTool};
use crate::api::infer::InferOptions;
use crate::common::errors::AppError;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait InferServiceExt: Send + Sync {
    async fn infer(
        &self,
        prompt: String,
        options: InferOptions,
    ) -> Result<(InvoiceTool, String), AppError>;
}

pub type InferServiceDyn = Arc<dyn InferServiceExt + Send + Sync>;

pub trait InferServiceFactoryExt: Send + Sync {
    fn create_service(&self, provider: InferMode) -> InferServiceDyn;
}

pub type InferServiceFactoryDyn = Arc<dyn InferServiceFactoryExt + Send + Sync>;

#[derive(Clone)]
pub struct InferServiceFactory {
    pub text_infer_service: InferServiceDyn,
    pub invoice_infer_service: InferServiceDyn,
}

impl InferServiceFactoryExt for InferServiceFactory {
    fn create_service(&self, provider: InferMode) -> InferServiceDyn {
        match provider {
            InferMode::Text => self.text_infer_service.clone(),
            InferMode::Invoice => self.invoice_infer_service.clone(),
        }
    }
}
