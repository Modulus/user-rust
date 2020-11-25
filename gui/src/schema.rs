table! {
    friends (user_id, friend_id) {
        user_id -> Int4,
        friend_id -> Int4,
        added -> Timestamp,
    }
}

table! {
    messages (id) {
        id -> Int4,
        header -> Varchar,
        message -> Varchar,
        sender_user_id -> Int4,
        receiver_user_id -> Int4,
        sent -> Timestamp,
        modified -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        comment -> Nullable<Varchar>,
        active -> Bool,
        pass_hash -> Varchar,
        created -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    friends,
    messages,
    users,
);
