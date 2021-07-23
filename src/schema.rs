table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        email_verified_at -> Nullable<Timestamp>,
        password -> Text,
        remember_token -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
