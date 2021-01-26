table! {
    use diesel::sql_types::*;
    use hangmanstructs::GameModeMapping;
    games (id) {
        id -> Int4,
        mode -> GameModeMapping,
        word -> Varchar,
        max_guesses -> Int4,
        creator_id -> Int4,
    }
}

table! {
    guess (id) {
        user_id -> Nullable<Int4>,
        id -> Int4,
        game_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        game_id -> Nullable<Int4>,
        password -> Varchar,
    }
}

joinable!(guess -> games (game_id));
joinable!(users -> games (game_id));

allow_tables_to_appear_in_same_query!(
    games,
    guess,
    users,
);
