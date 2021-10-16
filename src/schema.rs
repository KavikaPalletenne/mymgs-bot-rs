table! {
    classes (id) {
        id -> Int4,
        day_id -> Int4,
        period_number -> Int2,
        name -> Text,
        teacher -> Text,
    }
}

table! {
    days (id) {
        id -> Int4,
        timetable_id -> Int4,
        day_number -> Int2,
    }
}

table! {
    timetables (id) {
        id -> Int4,
        user_id -> Int8,
        fetched_date -> Date,
    }
}

table! {
    users (id) {
        id -> Int8,
        synergetic_user_id -> Int4,
        mgs_email -> Nullable<Text>,
        mgs_password -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    classes,
    days,
    timetables,
    users,
);
