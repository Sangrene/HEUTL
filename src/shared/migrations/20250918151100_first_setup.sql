-- Add migration script here
CREATE TABLE IF NOT EXISTS entity_subscriptions (id TEXT PRIMARY KEY, entity_sharing_id TEXT, created_at INTEGER, updated_at INTEGER, connected_app_id TEXT, jdm_transform TEXT);
CREATE TABLE IF NOT EXISTS entity_sharings (id TEXT PRIMARY KEY, name TEXT, created_at INTEGER, updated_at INTEGER, polling_infos TEXT, json_schema TEXT, connected_app_id TEXT);
CREATE TABLE IF NOT EXISTS connected_apps (id TEXT PRIMARY KEY, name TEXT, created_at INTEGER, updated_at INTEGER);