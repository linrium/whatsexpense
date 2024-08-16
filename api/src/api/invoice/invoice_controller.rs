use std::io;

use anyhow::anyhow;
use axum::extract::{Multipart, Path, State};
use axum::{Extension, Json};
use futures::TryStreamExt;
use tokio::io::BufWriter;
use tokio_util::io::StreamReader;
use utoipa::OpenApi;

use crate::api::infer::models::InferMode;
use crate::api::infer::InferOptions;
use crate::api::invoice::{
    Discount, Invoice, InvoiceError, PresignGetPayload, Tax, UploadImageBody, UploadImageInput,
    UploadedImage,
};
use crate::api::message::{CreateMessageInput, Message};
use crate::api::state::AppState;
use crate::api::user::User;
use crate::common::errors::AppError;
use crate::macros::object_id;

#[utoipa::path(
    post,
    path = "/upload",
    request_body(content = UploadImageBody, content_type = "multipart/form-data"),
    responses(
        (status = 200, description = "Upload successfully", body = [Message]),
    )
)]
pub async fn upload_invoice(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    mut multipart: Multipart,
) -> Result<Json<Vec<Message>>, AppError> {
    let invoice_service = state.invoice_service;
    let infer_service = state
        .infer_service_factory
        .create_service(InferMode::Invoice);
    let message_service = state.message_service;

    let field = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Unknown(anyhow!(e)))?;

    let Some(field) = field else {
        return Err(InvoiceError::NoAttachment.into());
    };

    let content_type = field
        .content_type()
        .ok_or::<AppError>(InvoiceError::UnsupportedContentType.into())?
        .to_string();

    if !content_type.starts_with("image/") {
        return Err(InvoiceError::UnsupportedContentType.into());
    }

    let body_with_io_error = field.map_err(|err| io::Error::new(io::ErrorKind::Other, err));
    let body_reader = StreamReader::new(body_with_io_error);
    futures::pin_mut!(body_reader);

    let mut file = BufWriter::new(vec![]);
    tokio::io::copy(&mut body_reader, &mut file)
        .await
        .map_err(|e| AppError::Unknown(anyhow!(e)))?;

    let UploadedImage {
        content,
        content_type,
        path,
    } = invoice_service
        .upload_image(
            file.into_inner(),
            UploadImageInput {
                user_id: object_id!(&user.id),
                content_type: content_type.to_string(),
            },
        )
        .await?;

    let categories = state.category_service.find().await?;
    let (invoice_tool, completion) = infer_service
        .infer(
            content.clone(),
            InferOptions {
                currencies: vec![user.currency.clone()],
                categories,
            },
        )
        .await?;

    let messages = message_service
        .create(CreateMessageInput {
            prompt: content,
            currencies: vec![user.currency],
            user_id: object_id!(&user.id),
            invoice_tool,
            completion,
            media_path: Some(path),
            media_type: Some(content_type),
        })
        .await?;

    Ok(Json(messages))
}

#[utoipa::path(
    get,
    path = "/presigned",
    responses(
        (status = 200, description = "Get presigned url successfully", body = PresignGetPayload),
    ),
    params(
        ("id" = ObjectId, Path, description = "Invoice database id to get invoice for"),
    )
)]
pub async fn presigned(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Path(invoice_id): Path<String>,
) -> Result<Json<PresignGetPayload>, AppError> {
    let invoice = state
        .invoice_service
        .find_by_id(object_id!(&invoice_id))
        .await?
        .ok_or(InvoiceError::NotFound)?;

    if invoice.user_id != user.id {
        return Err(InvoiceError::NotFound.into());
    }

    let path = invoice
        .media_path
        .ok_or(InvoiceError::NoAttachment)?
        .replace("invoices/", "");
    let presigned_post = state.r2_service.presign_get(path).await?;

    Ok(Json(PresignGetPayload {
        url: presigned_post,
    }))
}

#[derive(OpenApi)]
#[openapi(
    paths(upload_invoice, presigned),
    components(
        schemas(
            UploadImageBody,
            Message,
            Tax,
            Discount,
            Invoice,
            PresignGetPayload,
        )
    ),
    tags(
        (name = "crate::api::invoice", description = "Invoice API")
    )
)]
pub struct InvoiceApiDoc;
