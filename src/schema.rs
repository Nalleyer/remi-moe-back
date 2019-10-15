table! {
    games (id) {
        id -> Unsigned<Bigint>,
        title -> Varchar,
        link -> Text,
        create_time -> Timestamp,
    }
}
