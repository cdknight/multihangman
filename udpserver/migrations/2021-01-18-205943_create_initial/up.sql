-- Your SQL goes here

CREATE TYPE game_mode AS ENUM ('multi_guess', 'fastest_guess');

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE
);

CREATE TABLE guess (
    user_id INTEGER REFERENCES users(id),
    id SERIAL PRIMARY KEY
);


CREATE TABLE games (
    id SERIAL PRIMARY KEY,
    mode game_mode NOT NULL,
    word VARCHAR NOT NULL,
    max_guesses INTEGER,
    creator_id INTEGER REFERENCES users(id)
);

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

ALTER TABLE games ADD COLUMN guesses_id INTEGER REFERENCES games_guesses(id);
ALTER TABLE games ADD COLUMN players_id INTEGER REFERENCES games_players(id);
