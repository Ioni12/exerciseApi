use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// A row from the database. Sent back to the app as JSON.
#[derive(Serialize, FromRow)]
pub struct ExerciseLog {
    pub id: i32,
    pub exercise_name: String,
    pub reps: i32,
    pub created_at: DateTime<Utc>,
}

// The JSON the app sends when it wants to save a new log.
// No id or created_at: the database fills those in.
#[derive(Deserialize)]
pub struct NewExerciseLog {
    pub exercise_name: String,
    pub reps: i32,
}