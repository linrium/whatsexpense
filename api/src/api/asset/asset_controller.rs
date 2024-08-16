use crate::api::asset::{Country, Currency, Region, CURRENCIES, REGIONS};
use crate::common::errors::AppError;
use axum::Json;
use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "/currencies",
    responses(
        (status = 200, description = "Get currencies successfully", body = [Currency]),
    )
)]
pub(crate) async fn list_currencies() -> Result<Json<Vec<Currency>>, AppError> {
    Ok(Json(CURRENCIES.to_vec()))
}

#[utoipa::path(
    get,
    path = "/regions",
    responses(
        (status = 200, description = "Get regions successfully", body = [Region]),
    )
)]
pub(crate) async fn list_regions() -> Result<Json<Vec<Region<'static>>>, AppError> {
    Ok(Json(REGIONS.to_vec()))
}

#[utoipa::path(
    get,
    path = "/languages",
    responses(
        (status = 200, description = "Get languages successfully", body = [String], example = json!(["English"])),
    )
)]
pub(crate) async fn list_languages() -> Result<Json<Vec<String>>, AppError> {
    Ok(Json(vec!["English".to_string()]))
}

#[derive(OpenApi)]
#[openapi(
    paths(list_currencies, list_regions, list_languages),
    components(schemas(Currency, Country, Region)),
    tags(
        (name = "crate::api::asset", description = "Asset API")
    )
)]
pub struct AssetApiDoc;
