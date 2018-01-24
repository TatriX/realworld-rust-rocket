CREATE TABLE articles (
  id SERIAL PRIMARY KEY,
  slug TEXT NOT NULL UNIQUE,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  body TEXT NOT NULL,
  author INTEGER NOT NULL REFERENCES users,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
  favorites_count INTEGER NOT NULL DEFAULT 0
);

-- about tags: http://www.databasesoup.com/2015/01/tag-all-things.html
