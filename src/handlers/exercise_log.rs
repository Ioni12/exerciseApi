use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use sqlx::PgPool;

use crate::models::exercise_log::{ExerciseLog, NewExerciseLog};

// POST /logs
// Receives JSON, saves it in the database, returns the saved row.
pub async fn create_log(
    State(pool): State<PgPool>,
    Json(input): Json<NewExerciseLog>,
) -> Result<(StatusCode, Json<ExerciseLog>), StatusCode> {
    let log = sqlx::query_as::<_, ExerciseLog>(
        "INSERT INTO exercise_logs (exercise_name, reps)
         VALUES ($1, $2)
         RETURNING id, exercise_name, reps, created_at",
    )
    .bind(input.exercise_name)
    .bind(input.reps)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(log)))
}

// GET /logs
// Returns all saved logs, newest first.
pub async fn list_logs(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<ExerciseLog>>, StatusCode> {
    let logs = sqlx::query_as::<_, ExerciseLog>(
        "SELECT id, exercise_name, reps, created_at
         FROM exercise_logs
         ORDER BY created_at DESC",
    )
    .fetch_all(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(logs))
}

// DELETE /logs/{id}
// Deletes one log. 204 if deleted, 404 if the id doesn't exist.
pub async fn delete_log(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("DELETE FROM exercise_logs WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| {
            tracing::error!("delete_log failed: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(StatusCode::NO_CONTENT)
}