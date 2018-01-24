CREATE TABLE tags (
       id SERIAL PRIMARY KEY,
       name TEXT UNIQUE NOT NULL
);

CREATE TABLE article_tag (
       tag INTEGER REFERENCES tags ON DELETE CASCADE,
       article INTEGER REFERENCES articles ON DELETE CASCADE,
       PRIMARY KEY (tag, article)
);
