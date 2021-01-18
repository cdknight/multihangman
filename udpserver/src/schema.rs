table! {
    games (id) {
        id -> Int4,
        mode -> Game_mode,
        word -> Varchar,
        max_guesses -> Nullable<Int4>,
        creator_id -> Nullable<Int4>,
        guesses_id -> Nullable<Int4>,
        players_id -> Nullable<Int4>,
    }
}

table! {
    games_guesses (id) {
        id -> Int4,
        game_id -> Nullable<Int4>,
        guess_id -> Nullable<Int4>,
    }
}

table! {
    games_players (id) {
        id -> Int4,
        game_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
    }
}

table! {
    guess (id) {
        user_id -> Nullable<Int4>,
        id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
    }
}

joinable!(games -> users (creator_id));
joinable!(games_guesses -> guess (guess_id));
joinable!(games_players -> users (user_id));
joinable!(guess -> users (user_id));

allow_tables_to_appear_in_same_query!(
    games,
    games_guesses,
    games_players,
    guess,
    users,
);
