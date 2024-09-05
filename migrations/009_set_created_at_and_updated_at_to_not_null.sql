-- Make created_at and updated_at columns in the posts table NOT NULL
ALTER TABLE posts
ALTER COLUMN created_at SET NOT NULL,
ALTER COLUMN updated_at SET NOT NULL;

-- Make created_at and updated_at columns in the users table NOT NULL
ALTER TABLE users
ALTER COLUMN created_at SET NOT NULL,
ALTER COLUMN updated_at SET NOT NULL;

-- Make created_at and updated_at columns in the members table NOT NULL
ALTER TABLE members
ALTER COLUMN created_at SET NOT NULL,
ALTER COLUMN updated_at SET NOT NULL;

-- Make created_at and updated_at columns in the comments table NOT NULL
ALTER TABLE comments
ALTER COLUMN created_at SET NOT NULL,
ALTER COLUMN updated_at SET NOT NULL;

-- Make created_at and updated_at columns in the votes table NOT NULL
ALTER TABLE votes
ALTER COLUMN created_at SET NOT NULL,
ALTER COLUMN updated_at SET NOT NULL;
