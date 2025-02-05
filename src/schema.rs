// @generated automatically by Diesel CLI.

diesel::table! {
    conversations (id) {
        id -> Nullable<Text>,
        user_id -> Text,
        room_id -> Text,
        content -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    rooms (id) {
        id -> Nullable<Text>,
        name -> Text,
        last_message -> Nullable<Text>,
        participant_ids -> Nullable<Text>,
        created_at -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        phone -> Text,
        created_at -> Text,
    }
}

diesel::joinable!(conversations -> rooms (room_id));
diesel::joinable!(conversations -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(conversations, rooms, users,);
