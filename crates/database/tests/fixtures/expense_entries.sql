INSERT INTO expense_entry (id, coin_group_id, payer_id, total, created_at) VALUES
( -- John and Bill shared expenses; Bill paid 10 euros
    X'019b5551261d768c80a3d047d742e141', -- Expense entry Id
    X'019b37f428ff7b9dbe10b91f7a0dec74', -- Group Id
    X'019b3752b7d87a208bb28d0a44a1f661', -- Payer Id (Bill)
    1000,
    '2025-12-02T10:00:50Z'
);

INSERT INTO expense_entry_participant (expense_entry_id, participant_id) VALUES
(
    X'019b5551261d768c80a3d047d742e141',
    X'019b14ef290a70d9a2452a4723d9d44a' -- John
);
