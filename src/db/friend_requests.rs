use sqlx::PgPool;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::{FriendRequest, FriendRequestWithCallsigns, Friendship};

pub async fn create_friend_request(
    pool: &PgPool,
    from_user_id: Uuid,
    to_user_id: Uuid,
) -> Result<FriendRequestWithCallsigns, AppError> {
    let request = sqlx::query_as::<_, FriendRequestWithCallsigns>(
        r#"
        INSERT INTO friend_requests (from_user_id, to_user_id)
        VALUES ($1, $2)
        RETURNING
            friend_requests.id,
            friend_requests.from_user_id,
            (SELECT callsign FROM users WHERE id = friend_requests.from_user_id) as from_callsign,
            friend_requests.to_user_id,
            (SELECT callsign FROM users WHERE id = friend_requests.to_user_id) as to_callsign,
            friend_requests.status,
            friend_requests.requested_at,
            friend_requests.responded_at
        "#,
    )
    .bind(from_user_id)
    .bind(to_user_id)
    .fetch_one(pool)
    .await?;

    Ok(request)
}

pub async fn get_friend_request(
    pool: &PgPool,
    request_id: Uuid,
) -> Result<Option<FriendRequest>, AppError> {
    let request = sqlx::query_as::<_, FriendRequest>(
        r#"
        SELECT id, from_user_id, to_user_id, status, requested_at, responded_at
        FROM friend_requests
        WHERE id = $1
        "#,
    )
    .bind(request_id)
    .fetch_optional(pool)
    .await?;

    Ok(request)
}

pub async fn get_pending_request_between(
    pool: &PgPool,
    user_id_1: Uuid,
    user_id_2: Uuid,
) -> Result<Option<FriendRequest>, AppError> {
    let request = sqlx::query_as::<_, FriendRequest>(
        r#"
        SELECT id, from_user_id, to_user_id, status, requested_at, responded_at
        FROM friend_requests
        WHERE status = 'pending'
          AND ((from_user_id = $1 AND to_user_id = $2)
               OR (from_user_id = $2 AND to_user_id = $1))
        "#,
    )
    .bind(user_id_1)
    .bind(user_id_2)
    .fetch_optional(pool)
    .await?;

    Ok(request)
}

pub async fn are_friends(pool: &PgPool, user_id_1: Uuid, user_id_2: Uuid) -> Result<bool, AppError> {
    let friendship = sqlx::query_as::<_, Friendship>(
        r#"
        SELECT id, user_id, friend_id, created_at
        FROM friendships
        WHERE (user_id = $1 AND friend_id = $2)
           OR (user_id = $2 AND friend_id = $1)
        "#,
    )
    .bind(user_id_1)
    .bind(user_id_2)
    .fetch_optional(pool)
    .await?;

    Ok(friendship.is_some())
}

pub async fn accept_friend_request(
    pool: &PgPool,
    request_id: Uuid,
) -> Result<Option<FriendRequestWithCallsigns>, AppError> {
    let mut tx = pool.begin().await?;

    // Update request status
    let request = sqlx::query_as::<_, FriendRequestWithCallsigns>(
        r#"
        UPDATE friend_requests
        SET status = 'accepted', responded_at = now()
        WHERE id = $1 AND status = 'pending'
        RETURNING
            id,
            from_user_id,
            (SELECT callsign FROM users WHERE id = friend_requests.from_user_id) as from_callsign,
            to_user_id,
            (SELECT callsign FROM users WHERE id = friend_requests.to_user_id) as to_callsign,
            status,
            requested_at,
            responded_at
        "#,
    )
    .bind(request_id)
    .fetch_optional(&mut *tx)
    .await?;

    if let Some(ref req) = request {
        // Create bidirectional friendship entries
        sqlx::query(
            r#"
            INSERT INTO friendships (user_id, friend_id)
            VALUES ($1, $2), ($2, $1)
            ON CONFLICT DO NOTHING
            "#,
        )
        .bind(req.from_user_id)
        .bind(req.to_user_id)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(request)
}

pub async fn decline_friend_request(
    pool: &PgPool,
    request_id: Uuid,
) -> Result<Option<FriendRequest>, AppError> {
    let request = sqlx::query_as::<_, FriendRequest>(
        r#"
        UPDATE friend_requests
        SET status = 'declined', responded_at = now()
        WHERE id = $1 AND status = 'pending'
        RETURNING id, from_user_id, to_user_id, status, requested_at, responded_at
        "#,
    )
    .bind(request_id)
    .fetch_optional(pool)
    .await?;

    Ok(request)
}

/// Find registered users from a list of callsigns, excluding:
/// - The requesting user
/// - Users already friends with the requester
/// - Users with pending friend requests (either direction)
pub async fn find_suggested_friends(
    pool: &PgPool,
    user_id: Uuid,
    callsigns: &[String],
) -> Result<Vec<crate::models::User>, AppError> {
    if callsigns.is_empty() {
        return Ok(vec![]);
    }

    let users = sqlx::query_as::<_, crate::models::User>(
        r#"
        SELECT u.id, u.callsign, u.created_at
        FROM users u
        WHERE UPPER(u.callsign) = ANY(
            SELECT UPPER(unnest($2::text[]))
        )
        AND u.id != $1
        AND u.id NOT IN (
            SELECT friend_id FROM friendships WHERE user_id = $1
        )
        AND u.id NOT IN (
            SELECT to_user_id FROM friend_requests
            WHERE from_user_id = $1 AND status = 'pending'
            UNION
            SELECT from_user_id FROM friend_requests
            WHERE to_user_id = $1 AND status = 'pending'
        )
        "#,
    )
    .bind(user_id)
    .bind(callsigns)
    .fetch_all(pool)
    .await?;

    Ok(users)
}
