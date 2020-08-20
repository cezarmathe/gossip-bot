table! {
    gossipers (id) {
        id -> Integer,
        discord_id -> Binary,
        preferred_guild -> Nullable<Binary>,
    }
}

table! {
    gossips (id) {
        id -> Integer,
        msg -> Text,
        added -> Timestamp,
        guild_id -> Binary,
    }
}

allow_tables_to_appear_in_same_query!(gossipers, gossips,);
