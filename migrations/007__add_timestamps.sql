-- Add created_at and updated_at columns to posts table
ALTER TABLE posts
ADD COLUMN created_at TIMESTAMPTZ DEFAULT NOW(),
ADD COLUMN updated_at TIMESTAMPTZ DEFAULT NOW();

-- Add created_at and updated_at columns to votes table
ALTER TABLE votes
ADD COLUMN created_at TIMESTAMPTZ DEFAULT NOW(),
ADD COLUMN updated_at TIMESTAMPTZ DEFAULT NOW();

-- Add created_at and updated_at columns to members table
ALTER TABLE members
ADD COLUMN created_at TIMESTAMPTZ DEFAULT NOW(),
ADD COLUMN updated_at TIMESTAMPTZ DEFAULT NOW();

-- Add created_at and updated_at columns to comments table
ALTER TABLE comments
ADD COLUMN created_at TIMESTAMPTZ DEFAULT NOW(),
ADD COLUMN updated_at TIMESTAMPTZ DEFAULT NOW();
