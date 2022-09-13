// @generated automatically by Diesel CLI.

diesel::table! {
    tweets (id) {
        id -> Int4,
        message -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        login_session -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    tweets,
    users,
);
