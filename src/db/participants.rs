use sqlx::PgPool;
use uuid::Uuid;

use crate::auth::generate_device_token;
use crate::error::AppError;
use crate::models::{ChallengeParticipant, Participant};

pub async fn get_or_create_participant(
    pool: &PgPool,
    callsign: &str,
    device_name: Option<&str>,
) -> Result<(Participant, bool), AppError> {
    let existing = sqlx::query_as::<_, Participant>(
        r#"
        SELECT id, callsign, device_token, device_name, created_at, last_seen_at
        FROM participants
        WHERE callsign = $1
        LIMIT 1
        "#,
    )
    .bind(callsign)
    .fetch_optional(pool)
    .await?;

    if let Some(p) = existing {
        return Ok((p, false));
    }

    let id = Uuid::new_v4();
    let device_token = generate_device_token();

    let participant = sqlx::query_as::<_, Participant>(
        r#"
        INSERT INTO participants (id, callsign, device_token, device_name)
        VALUES ($1, $2, $3, $4)
        RETURNING id, callsign, device_token, device_name, created_at, last_seen_at
        "#,
    )
    .bind(id)
    .bind(callsign)
    .bind(&device_token)
    .bind(device_name)
    .fetch_one(pool)
    .await?;

    Ok((participant, true))
}

pub async fn get_participant_by_token(
    pool: &PgPool,
    token: &str,
) -> Result<Option<Participant>, AppError> {
    let participant = sqlx::query_as::<_, Participant>(
        r#"
        SELECT id, callsign, device_token, device_name, created_at, last_seen_at
        FROM participants
        WHERE device_token = $1
        "#,
    )
    .bind(token)
    .fetch_optional(pool)
    .await?;

    Ok(participant)
}

pub async fn join_challenge(
    pool: &PgPool,
    challenge_id: Uuid,
    callsign: &str,
    invite_token: Option<&str>,
) -> Result<ChallengeParticipant, AppError> {
    let id = Uuid::new_v4();

    let participation = sqlx::query_as::<_, ChallengeParticipant>(
        r#"
        INSERT INTO challenge_participants (id, challenge_id, callsign, invite_token)
        VALUES ($1, $2, $3, $4)
        RETURNING id, challenge_id, callsign, invite_token, joined_at, status
        "#,
    )
    .bind(id)
    .bind(challenge_id)
    .bind(callsign)
    .bind(invite_token)
    .fetch_one(pool)
    .await
    .map_err(|e| {
        if let sqlx::Error::Database(ref db_err) = e {
            if db_err.constraint() == Some("challenge_participants_challenge_id_callsign_key") {
                return AppError::AlreadyJoined;
            }
        }
        AppError::Database(e)
    })?;

    Ok(participation)
}

pub async fn get_participation(
    pool: &PgPool,
    challenge_id: Uuid,
    callsign: &str,
) -> Result<Option<ChallengeParticipant>, AppError> {
    let participation = sqlx::query_as::<_, ChallengeParticipant>(
        r#"
        SELECT id, challenge_id, callsign, invite_token, joined_at, status
        FROM challenge_participants
        WHERE challenge_id = $1 AND callsign = $2
        "#,
    )
    .bind(challenge_id)
    .bind(callsign)
    .fetch_optional(pool)
    .await?;

    Ok(participation)
}

pub async fn leave_challenge(
    pool: &PgPool,
    challenge_id: Uuid,
    callsign: &str,
) -> Result<bool, AppError> {
    let result = sqlx::query(
        r#"
        UPDATE challenge_participants
        SET status = 'left'
        WHERE challenge_id = $1 AND callsign = $2 AND status = 'active'
        "#,
    )
    .bind(challenge_id)
    .bind(callsign)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}

pub async fn revoke_tokens(pool: &PgPool, callsign: &str) -> Result<u64, AppError> {
    let result = sqlx::query("DELETE FROM participants WHERE callsign = $1")
        .bind(callsign)
        .execute(pool)
        .await?;

    Ok(result.rows_affected())
}
