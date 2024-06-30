-- Add migration script here
CREATE TYPE gender AS ENUM(
    'female',
    'male',
    'unknown'
);

CREATE TABLE user_stats(
    email varchar(128) NOT NULL PRIMARY KEY,
    name varchar(64) NOT NULL,
    gender gender DEFAULT 'unknown',
    created_at timestamptz DEFAULT CURRENT_TIMESTAMP,
    last_visited_at timestamptz,
    last_watched_at timestamptz,
    recent_watched int[],
    viewed_but_not_started int[],
    started_but_not_finished int[],
    finished int[],
    last_email_notification timestamptz,
    last_in_app_notification timestamptz,
    last_sms_notification timestamptz
);

DROP INDEX IF EXISTS user_stats_created_at_idx;

DROP INDEX IF EXISTS user_stats_last_visited_at_idx;

DROP INDEX IF EXISTS user_stats_last_watched_at_idx;

DROP INDEX IF EXISTS user_stats_recent_watched_idx;

DROP INDEX IF EXISTS user_stats_viewed_but_not_started_idx;

DROP INDEX IF EXISTS user_stats_started_but_not_finished_idx;

DROP INDEX IF EXISTS user_stats_last_email_notification_idx;

DROP INDEX IF EXISTS user_stats_last_in_app_notification_idx;

DROP INDEX IF EXISTS user_stats_last_sms_notification_idx;
