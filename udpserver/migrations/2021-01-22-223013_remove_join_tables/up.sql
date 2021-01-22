-- Your SQL goes here

ALTER TABLE users ADD COLUMN game_id INTEGER REFERENCES games(id);
ALTER TABLE guess ADD COLUMN game_id INTEGER REFERENCES games(id);
ALTER TABLE games DROP COLUMN players_id;
ALTER TABLE games DROP COLUMN guesses_id;

DROP TABLE games_guesses CASCADE;
DROP TABLE games_players CASCADE;
