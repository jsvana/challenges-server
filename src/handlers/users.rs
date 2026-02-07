use axum::extract::{Query, State};
use serde::Deserialize;
use sqlx::PgPool;

use crate::db;
use crate::error::AppError;
use crate::extractors::Json;
use crate::models::UserSearchResponse;

use super::DataResponse;

#[derive(Debug, Deserialize)]
pub struct SearchUsersQuery {
    pub q: String,
}

/// GET /v1/users/search?q=...
/// Search for users by callsign (public, no auth required)
pub async fn search_users(
    State(pool): State<PgPool>,
    Query(query): Query<SearchUsersQuery>,
) -> Result<Json<DataResponse<Vec<UserSearchResponse>>>, AppError> {
    if query.q.len() < 2 {
        return Ok(Json(DataResponse { data: vec![] }));
    }

    let users = db::search_users(&pool, &query.q, 20).await?;

    let results: Vec<UserSearchResponse> = users.into_iter().map(|u| u.into()).collect();

    Ok(Json(DataResponse { data: results }))
}
