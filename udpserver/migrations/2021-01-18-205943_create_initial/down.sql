-- This file should undo anything in `up.sql`

DROP TABLE games CASCADE;
DROP TABLE users CASCADE;

DROP TABLE games_guesses;
DROP TABLE games_players;
DROP TABLE guess;
DROP TYPE game_mode;
