table! {
    messages (id) {
        id -> Int4,
        header -> Varchar,
        message -> Varchar,
        sender_user_id -> Nullable<Int4>,
        receiver_user_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        comment -> Nullable<Varchar>,
        active -> Bool,
        pass_hash -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
