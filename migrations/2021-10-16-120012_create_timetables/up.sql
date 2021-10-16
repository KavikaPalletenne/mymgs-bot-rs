CREATE TABLE timetables (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    fetched_date DATE NOT NULL
)