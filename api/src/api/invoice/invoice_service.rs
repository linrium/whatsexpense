use std::sync::Arc;

use async_trait::async_trait;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use bson::doc;
use bson::oid::ObjectId;
use mime2ext::mime2ext;
use mongodb::ClientSession;
use uuid::Uuid;

use crate::api::invoice::*;
use crate::common::errors::AppError;
use crate::services::gcp::vision::VisionServiceDyn;
use crate::services::r2::R2ServiceDyn;
use crate::settings::InvoiceConfig;

#[async_trait]
pub trait InvoiceServiceExt: Send + Sync {
    async fn find_by_message_id(&self, message_id: ObjectId) -> Result<Option<Invoice>, AppError>;
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Invoice>, AppError>;
    async fn insert_one_with_session(
        &self,
        input: CreateInvoiceInput,
        session: &mut ClientSession,
    ) -> Result<Invoice, AppError>;
    async fn upload_image(
        &self,
        content: Vec<u8>,
        input: UploadImageInput,
    ) -> Result<UploadedImage, AppError>;
    async fn delete_by_id_with_session(
        &self,
        id: ObjectId,
        session: &mut ClientSession,
    ) -> Result<bool, AppError>;
}

pub type InvoiceServiceDyn = Arc<dyn InvoiceServiceExt + Send + Sync>;

#[derive(Clone)]
pub struct InvoiceService {
    pub repo: InvoiceRepoDyn,
    pub r2_service: R2ServiceDyn,
    pub gcp_vision_service: VisionServiceDyn,
    pub config: InvoiceConfig,
}

#[async_trait]
impl InvoiceServiceExt for InvoiceService {
    async fn find_by_message_id(&self, message_id: ObjectId) -> Result<Option<Invoice>, AppError> {
        self.repo
            .find_one(doc! { "messageId": message_id })
            .await
            .map(|v| v.map(Into::into))
            .map_err(|e| e.into())
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Invoice>, AppError> {
        self.repo
            .find_by_id(id)
            .await
            .map(|v| v.map(Into::into))
            .map_err(|e| e.into())
    }

    async fn insert_one_with_session(
        &self,
        input: CreateInvoiceInput,
        session: &mut ClientSession,
    ) -> Result<Invoice, AppError> {
        self.repo
            .insert_one_with_session(
                CreateInvoiceData {
                    id: input.id,
                    user_id: input.user_id,
                    message_id: input.message_id,
                    taxes: input.taxes,
                    discounts: input.discounts,
                    subtotal: input.subtotal,
                    total: input.total,
                    currency: input.currency,
                    card_number: input.card_number,
                    media_path: input.media_path,
                    media_type: input.media_type,
                },
                session,
            )
            .await
            .map(Into::into)
            .map_err(|e| e.into())
    }

    async fn upload_image(
        &self,
        content: Vec<u8>,
        input: UploadImageInput,
    ) -> Result<UploadedImage, AppError> {
        let invoice_id = Uuid::new_v4().to_string();
        let extension =
            mime2ext(&input.content_type).ok_or(InvoiceError::UnsupportedContentType)?;
        let path = format!("{}/{}.{}", input.user_id, invoice_id, extension);

        let content = content.as_slice();
        let encoded = BASE64_STANDARD.encode(content);

        let (content, path) = tokio::join!(
            self.gcp_vision_service.detect_text(encoded),
            self.r2_service.upload_object(
                self.config.bucket.clone(),
                path,
                content,
                &input.content_type
            ),
        );

        let path = path.map_err(|e| AppError::from(e))?;
        let content = content.map_err(|e| AppError::from(e))?;

        Ok(UploadedImage {
            path,
            content_type: input.content_type,
            content,
        })
    }

    async fn delete_by_id_with_session(
        &self,
        id: ObjectId,
        session: &mut ClientSession,
    ) -> Result<bool, AppError> {
        self.repo
            .delete_by_id_with_session(id, session)
            .await
            .map_err(|e| e.into())
    }
}
