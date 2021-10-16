CREATE TABLE days (
    id SERIAL PRIMARY KEY,
    timetable_id INTEGER NOT NULL,
    day_number SMALLINT NOT NULL
)