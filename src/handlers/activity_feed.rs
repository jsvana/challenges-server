use axum::{
    extract::{Extension, Query, State},
    http::StatusCode,
};

use crate::extractors::Json;
use sqlx::PgPool;

use crate::auth::AuthContext;
use crate::error::AppError;

use super::DataResponse;

/// POST /v1/activities
/// Report a notable activity (stub: accepts and returns the activity)
pub async fn report_activity(
    State(_pool): State<PgPool>,
    Extension(auth): Extension<AuthContext>,
    Json(body): Json<serde_json::Value>,
) -> Result<(StatusCode, Json<DataResponse<serde_json::Value>>), AppError> {
    // Stub: echo back with an id and the caller's callsign
    let mut response = body.clone();
    if let Some(obj) = response.as_object_mut() {
        obj.insert("id".to_string(), serde_json::json!(uuid::Uuid::new_v4()));
        obj.insert("callsign".to_string(), serde_json::json!(auth.callsign));
    }

    Ok((StatusCode::CREATED, Json(DataResponse { data: response })))
}

#[derive(serde::Deserialize)]
#[allow(dead_code)]
pub struct FeedQuery {
    pub limit: Option<i64>,
    pub filter: Option<String>,
    pub before: Option<String>,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedResponse {
    pub items: Vec<serde_json::Value>,
    pub pagination: FeedPagination,
}

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedPagination {
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

/// GET /v1/feed
/// Get activity feed (stub: returns empty feed)
pub async fn get_feed(
    State(_pool): State<PgPool>,
    Extension(_auth): Extension<AuthContext>,
    Query(_params): Query<FeedQuery>,
) -> Result<Json<DataResponse<FeedResponse>>, AppError> {
    Ok(Json(DataResponse {
        data: FeedResponse {
            items: vec![],
            pagination: FeedPagination {
                has_more: false,
                next_cursor: None,
            },
        },
    }))
}

/// GET /v1/clubs
/// Get clubs for user (stub: returns empty list)
pub async fn get_clubs(
    State(_pool): State<PgPool>,
    Extension(_auth): Extension<AuthContext>,
) -> Result<Json<DataResponse<Vec<serde_json::Value>>>, AppError> {
    Ok(Json(DataResponse { data: vec![] }))
}

/// GET /v1/clubs/:id
/// Get club details (stub: returns not found)
pub async fn get_club_details(
    State(_pool): State<PgPool>,
    Extension(_auth): Extension<AuthContext>,
    axum::extract::Path(_club_id): axum::extract::Path<uuid::Uuid>,
) -> Result<Json<DataResponse<serde_json::Value>>, AppError> {
    Err(AppError::Validation {
        message: "Clubs not yet implemented".to_string(),
    })
}
