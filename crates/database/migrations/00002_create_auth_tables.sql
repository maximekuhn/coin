CREATE TABLE auth_entry (
    id BLOB(16) PRIMARY KEY,
    user_id BLOB(16) NOT NULL,
    hashed_password BLOB NOT NULL,
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
);

CREATE TABLE auth_session (
    id BLOB(128) PRIMARY KEY,
    auth_entry_id BLOB(16) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    FOREIGN KEY (auth_entry_id) REFERENCES auth_entry(id) ON DELETE CASCADE
);
