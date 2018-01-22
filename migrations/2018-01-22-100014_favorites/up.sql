CREATE TABLE favorites (
       "user" INTEGER REFERENCES users ON DELETE CASCADE,
       article INTEGER REFERENCES articles ON DELETE CASCADE,
       PRIMARY KEY ("user", article)
);
