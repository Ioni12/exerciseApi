use axum::{Json, extract::State, http::StatusCode};
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