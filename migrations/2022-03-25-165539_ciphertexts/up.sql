-- Your SQL goes here
CREATE TABLE ciphertexts (
    resource_id INTEGER PRIMARY KEY,
    resource_type TEXT NOT NULL,
    ciphertext TEXT NOT NULL
)