CREATE TABLE follows (
       follower INTEGER REFERENCES users ON DELETE CASCADE,
       followed INTEGER REFERENCES users ON DELETE CASCADE,
       CHECK (follower != followed),
       PRIMARY KEY(follower, followed)
);
