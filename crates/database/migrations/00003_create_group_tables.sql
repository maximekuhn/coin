CREATE TABLE coin_group (
    id BLOB(16) PRIMARY KEY,
    name TEXT NOT NULL,
    owner_id BLOB(16) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    UNIQUE (name, owner_id),
    FOREIGN KEY (owner_id) REFERENCES user(id) ON DELETE CASCADE
);

CREATE TABLE coin_group_member (
    coin_group_id BLOB(16) NOT NULL,
    member_id BLOB(16) NOT NULL,
    PRIMARY KEY (coin_group_id, member_id),
    FOREIGN KEY (coin_group_id) REFERENCES coin_group(id) ON DELETE CASCADE,
    FOREIGN KEY (member_id) REFERENCES user(id) ON DELETE CASCADE
);
