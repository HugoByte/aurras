// @generated automatically by Diesel CLI.

diesel::table! {
    action_details (id) {
        id -> Int4,
        rule -> Varchar,
        action -> Varchar,
        trigger -> Varchar,
        active_status -> Bool,
        url -> Varchar,
        auth -> Varchar,
        namespace -> Varchar,
        user_id -> Uuid,
    }
}

diesel::table! {
    userss (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        password_hash -> Varchar,
        full_name -> Varchar,
        actions -> Array<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(action_details -> userss (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    action_details,
    userss,
);
