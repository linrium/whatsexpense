use crate::api::category::Category;
use crate::api::state::AppState;
use crate::common::errors::AppError;
use axum::extract::State;
use axum::Json;
use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "",
    responses(
        (status = 200, description = "Get categories successfully", body = [Category]),
    )
)]
pub(crate) async fn list_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<Category>>, AppError> {
    let categories = state.category_service.find().await?;

    Ok(Json(categories))
}

#[derive(OpenApi)]
#[openapi(
    paths(list_categories),
    components(
        schemas(
            Category,
        )
    ),
    tags(
        (name = "crate::api::category", description = "Category API")
    )
)]
pub struct CategoryApiDoc;
