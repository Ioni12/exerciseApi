CREATE TABLE exercise_logs (
    id SERIAL PRIMARY KEY,
    exercise_name TEXT NOT NULL,
    reps INTEGER NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
