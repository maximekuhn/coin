CREATE TABLE expense_entry (
    id BLOB(16) PRIMARY KEY,
    expense_id BLOB(16) NOT NULL,
    coin_group_id BLOB(16) NOT NULL,
    payer_id BLOB(16) NOT NULL,
    status BLOB(16),
    total INTEGER NOT NULL,
    author_id BLOB(16) NOT NULL,
    occurred_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL,
    FOREIGN KEY (coin_group_id) REFERENCES coin_group(id) ON DELETE CASCADE,
    FOREIGN KEY (payer_id) REFERENCES user(id) ON DELETE CASCADE,
    FOREIGN KEY (author_id) REFERENCES user(id) ON DELETE CASCADE
);

CREATE TABLE expense_entry_participant (
    expense_entry_id BLOB(16) NOT NULL,
    participant_id BLOB(16) NOT NULL,
    PRIMARY KEY (expense_entry_id, participant_id),
    FOREIGN KEY (expense_entry_id) REFERENCES expense_entry(id) ON DELETE CASCADE,
    FOREIGN KEY (participant_id) REFERENCES user(id) ON DELETE CASCADE
);
