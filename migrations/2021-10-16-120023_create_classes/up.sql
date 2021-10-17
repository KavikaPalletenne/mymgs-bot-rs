create table classes (
    id SERIAL PRIMARY KEY,
    timetable_id INTEGER NOT NULL,
    day_number SMALLINT NOT NULL,
    period_number SMALLINT NOT NULL,
    name TEXT NOT NULL,
    teacher TEXT NOT NULL
)