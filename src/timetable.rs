// TODO: Find a good ORM library for Rust
use diesel::associations;


pub struct Timetable {
    id: u32,
    user_id: u32,
    fetched_date: String, // TODO: Find a date library (saving date to SQL, probably comes with ORM) and a current time library
    // TODO: Either do it this way (like I have done in the current bot) or find a way to do it properly (with an array for each day or array of arrays) depending on the Rust ORM.
}

#[belongs_to(Timetable)]
#[table_name = "tt_day"]
pub struct Day {
    id: u32,
    timetable_id: u32,
    day_number: u32, // Day 1 would have the value "1"

}
#[belongs_to(Day)]
#[table]
pub struct Class {
    id: u32,
    timetable_day_id: u32,
    period_number: u32, // The first period of the day would have value "1"

    name: String,
    teacher: String,
}