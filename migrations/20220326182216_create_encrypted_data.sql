-- Add migration script here
CREATE TABLE IF NOT EXISTS encrypteddata (
    resource_id INTEGER PRIMARY KEY,
    resource_type TEXT NOT NULL,
    ciphertext TEXT NOT NULL
);