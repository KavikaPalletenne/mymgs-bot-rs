create table classes (
    id SERIAL PRIMARY KEY,
    day_id INTEGER NOT NULL,
    period_number SMALLINT NOT NULL,
    name TEXT NOT NULL,
    teacher TEXT NOT NULL
)