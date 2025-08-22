-- Initialization script for PostgreSQL
-- This runs when the container starts for the first time

-- Create extensions that might be useful
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Set timezone
SET timezone = 'UTC';

-- Create some useful functions for development
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Grant necessary permissions
GRANT ALL PRIVILEGES ON DATABASE birthday_bot TO birthday_bot;
GRANT ALL PRIVILEGES ON SCHEMA public TO birthday_bot;

-- Show startup info
SELECT 'Birthday Bot PostgreSQL initialized successfully!' as status;
