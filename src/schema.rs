table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        comment -> Nullable<Varchar>,
        active -> Bool,
        pass_hash -> Varchar,
    }
}
