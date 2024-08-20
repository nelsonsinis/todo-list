// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Uuid,
        title -> Varchar,
        description -> Nullable<Text>,
        checked -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Varchar,
        email -> Varchar,
        username -> Nullable<Varchar>,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
