INSERT INTO users (id, email, first_name, last_name, username, password)
VALUES
    (1, 'bobvance@gmail.com', 'Bob', 'Vance', 'bobvance', '123'),
    (2, 'tonysoprano@gmail.com', 'Tony', 'Soprano', 'tonysoprano', '123'),
    (3, 'billburr@gmail.com', 'Bill', 'Burr', 'billburr', '123');

INSERT INTO members (id, user_id)
VALUES
    (1, 1),
    (2, 2),
    (3, 3);

INSERT INTO posts (id, member_id, title, content, post_type, date_created)
VALUES
    (1, 1, 'First post!', 'This is Bob Vance''s first post', 'Text', NOW()),
    (2, 1, 'Second post!', 'This is Bob''s second post', 'Text', NOW()),
    (3, 2, 'Another post', 'This is Tony''s first post', 'Text', NOW()),
    (4, 2, 'Links', 'This is a link post', 'Link', NOW());

INSERT INTO votes (id, post_id, vote_type, member_id)
VALUES
    (1, 1, 'Upvote', 1),
    (2, 2, 'Upvote', 1),
    (3, 3, 'Upvote', 2),
    (4, 4, 'Upvote', 2),
    (5, 3, 'Upvote', 1),
    (6, 2, 'Downvote', 3);

INSERT INTO comments (id, text, member_id, post_id, parent_comment_id)
VALUES
    (1, 'I posted this!', 1, 1, NULL),
    (2, 'Nice', 2, 2, NULL);
