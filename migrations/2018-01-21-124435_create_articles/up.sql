CREATE TABLE articles (
  id SERIAL PRIMARY KEY,
  slug TEXT NOT NULL UNIQUE,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  body TEXT NOT NULL,
  author INTEGER REFERENCES users,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),
  favorites_count INTEGER DEFAULT 0
);

-- about tags: http://www.databasesoup.com/2015/01/tag-all-things.html
