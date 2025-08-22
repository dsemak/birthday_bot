-- Initial schema for birthday bot v2
-- Migration: 001_initial_schema
-- Created: 2024-01-01

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create enum types
CREATE TYPE chat_type AS ENUM ('private', 'group', 'supergroup', 'channel');
CREATE TYPE user_role AS ENUM ('maintainer', 'admin', 'member', 'restricted');

-- Chats/groups table
CREATE TABLE chats (
    chat_id BIGINT PRIMARY KEY,
    chat_type chat_type NOT NULL,
    title VARCHAR(255),
    username VARCHAR(100),
    is_active BOOLEAN NOT NULL DEFAULT true,
    timezone VARCHAR(50) NOT NULL DEFAULT 'UTC',
    language_code VARCHAR(10) NOT NULL DEFAULT 'ru',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Users table
CREATE TABLE users (
    user_id BIGINT PRIMARY KEY,
    username VARCHAR(100),
    first_name VARCHAR(100),
    last_name VARCHAR(100),
    language_code VARCHAR(10),
    is_bot BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Chat members table (for tracking permissions)
CREATE TABLE chat_members (
    chat_id BIGINT NOT NULL REFERENCES chats(chat_id) ON DELETE CASCADE,
    user_id BIGINT NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    role user_role NOT NULL DEFAULT 'member',
    joined_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (chat_id, user_id)
);

-- Birthdays table
CREATE TABLE birthdays (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    chat_id BIGINT NOT NULL REFERENCES chats(chat_id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    birth_month SMALLINT NOT NULL CHECK (birth_month BETWEEN 1 AND 12),
    birth_day SMALLINT NOT NULL CHECK (birth_day BETWEEN 1 AND 31),
    birth_year SMALLINT CHECK (birth_year IS NULL OR (birth_year BETWEEN 1900 AND 2100)),
    username VARCHAR(100),
    notes TEXT,
    created_by BIGINT REFERENCES users(user_id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(chat_id, name, birth_month, birth_day)
);

-- Notification settings table
CREATE TABLE notification_settings (
    chat_id BIGINT PRIMARY KEY REFERENCES chats(chat_id) ON DELETE CASCADE,
    notify_time TIME NOT NULL DEFAULT '07:00:00',
    notify_days_before INTEGER[] NOT NULL DEFAULT '{0}',
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    message_template TEXT NOT NULL DEFAULT '🎉 Сегодня день рождения у {name}! {username}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Rate limiting table
CREATE TABLE rate_limits (
    user_id BIGINT NOT NULL,
    chat_id BIGINT,
    action_type VARCHAR(50) NOT NULL,
    window_start TIMESTAMP WITH TIME ZONE NOT NULL,
    request_count INTEGER NOT NULL DEFAULT 1,
    is_blocked BOOLEAN NOT NULL DEFAULT false,
    blocked_until TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (user_id, chat_id, action_type)
);

-- Conversation states table (for managing bot conversations)
CREATE TABLE conversation_states (
    user_id BIGINT NOT NULL,
    chat_id BIGINT NOT NULL,
    state JSONB NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, chat_id)
);

-- Audit log table (for tracking important actions)
CREATE TABLE audit_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id BIGINT,
    chat_id BIGINT,
    action VARCHAR(100) NOT NULL,
    details JSONB,
    ip_address INET,
    user_agent TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Metrics table (for storing bot usage statistics)
CREATE TABLE metrics (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    metric_name VARCHAR(100) NOT NULL,
    metric_value DECIMAL NOT NULL,
    labels JSONB,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_chats_is_active ON chats(is_active);
CREATE INDEX idx_chats_created_at ON chats(created_at);

CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_created_at ON users(created_at);

CREATE INDEX idx_chat_members_user_id ON chat_members(user_id);
CREATE INDEX idx_chat_members_role ON chat_members(role);

CREATE INDEX idx_birthdays_chat_id ON birthdays(chat_id);
CREATE INDEX idx_birthdays_birth_date ON birthdays(birth_month, birth_day);
CREATE INDEX idx_birthdays_created_by ON birthdays(created_by);
CREATE INDEX idx_birthdays_created_at ON birthdays(created_at);

CREATE INDEX idx_rate_limits_user_id ON rate_limits(user_id);
CREATE INDEX idx_rate_limits_window_start ON rate_limits(window_start);
CREATE INDEX idx_rate_limits_is_blocked ON rate_limits(is_blocked);

CREATE INDEX idx_conversation_states_expires_at ON conversation_states(expires_at);

CREATE INDEX idx_audit_log_user_id ON audit_log(user_id);
CREATE INDEX idx_audit_log_chat_id ON audit_log(chat_id);
CREATE INDEX idx_audit_log_action ON audit_log(action);
CREATE INDEX idx_audit_log_created_at ON audit_log(created_at);

CREATE INDEX idx_metrics_metric_name ON metrics(metric_name);
CREATE INDEX idx_metrics_timestamp ON metrics(timestamp);

-- Create function to automatically update updated_at columns
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create triggers for updated_at columns
CREATE TRIGGER update_chats_updated_at BEFORE UPDATE ON chats 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_birthdays_updated_at BEFORE UPDATE ON birthdays 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_notification_settings_updated_at BEFORE UPDATE ON notification_settings 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_conversation_states_updated_at BEFORE UPDATE ON conversation_states 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Create function to clean up expired conversation states
CREATE OR REPLACE FUNCTION cleanup_expired_conversation_states()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM conversation_states WHERE expires_at < NOW();
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- Create function to clean up old rate limit records
CREATE OR REPLACE FUNCTION cleanup_old_rate_limits()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM rate_limits 
    WHERE window_start < NOW() - INTERVAL '24 hours'
    AND NOT is_blocked;
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;


