create table classes (
    id SERIAL PRIMARY KEY,
    timetable_id INTEGER NOT NULL,
    day_number SMALLINT NOT NULL,
    period_number SMALLINT NOT NULL,
    name VARCHAR NOT NULL,
    teacher VARCHAR NOT NULL
)