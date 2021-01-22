
diesel::table! {
    use diesel::sql_types::*;
    use hangmanstructs::GameModeMapping;
    games (id) {
        id -> Int4,
        mode -> GameModeMapping,
        word -> Varchar,
        max_guesses -> Int4,
        creator_id -> Int4,
        guesses_id -> Nullable<Int4>,
        players_id -> Nullable<Int4>,
    }
}

diesel::table! {
    games_guesses (id) {
        id -> Int4,
        game_id -> Int4,
        guess_id -> Int4,
    }
}

diesel::table! {
    games_players (id) {
        id -> Int4,
        game_id -> Int4,
        user_id -> Int4,
    }
}

diesel::table! {
    guess (id) {
        user_id -> Int4,
        id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
    }
}

diesel::joinable!(games -> users (creator_id));
diesel::joinable!(games -> games_guesses (creator_id));
diesel::joinable!(games -> games_players (creator_id));
diesel::joinable!(games_guesses -> guess (guess_id));
diesel::joinable!(games_players -> users (user_id));
diesel::joinable!(guess -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    games,
    games_guesses,
    games_players,
    guess,
    users,
);
