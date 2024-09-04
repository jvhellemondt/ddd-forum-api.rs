CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    post_id INT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    member_id INT NOT NULL REFERENCES members(id) ON DELETE CASCADE,
    text TEXT NOT NULL,
    parent_comment_id INT REFERENCES comments(id) ON DELETE SET NULL
);
