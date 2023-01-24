// @generated automatically by Diesel CLI.

diesel::table! {
    userss (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        full_name -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
