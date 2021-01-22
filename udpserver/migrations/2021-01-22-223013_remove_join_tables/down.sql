-- This file should undo anything in `up.sql`
ALTER TABLE users DROP COLUMN game_id;
ALTER TABLE guess DROP COLUMN game_id;

CREATE TABLE games_guesses (
    id SERIAL PRIMARY KEY,
    game_id INTEGER REFERENCES games(id),
    guess_id INTEGER REFERENCES guess(id)
);

CREATE TABLE games_players (
   id SERIAL PRIMARY KEY,
   game_id INTEGER REFERENCES games(id),
   user_id INTEGER REFERENCES users(id)
);

ALTER TABLE games ADD COLUMN IF NOT EXISTS guesses_id INTEGER REFERENCES games_guesses(id);
ALTER TABLE games ADD COLUMN IF NOT EXISTS players_id INTEGER REFERENCES games_players(id);
