CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    member_id INT NOT NULL REFERENCES members(id) ON DELETE CASCADE,
    post_type TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    date_created TIMESTAMPTZ DEFAULT NOW()
);
