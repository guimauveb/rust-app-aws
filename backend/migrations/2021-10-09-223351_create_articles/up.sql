CREATE TABLE IF NOT EXISTS articles
(
   id            SERIAL PRIMARY KEY,
   title         TEXT NOT NULL,
   pub_date      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
   published     BOOLEAN NOT NULL DEFAULT FALSE,
   headline      TEXT NOT NULL,
   image         TEXT NOT NULL,
   content       TEXT NOT NULL
);
