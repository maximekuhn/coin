INSERT INTO expense_entry 
(id, expense_id, coin_group_id, payer_id, status, total, author_id, occurred_at, created_at)
 VALUES
( -- John and Bill shared expenses; Bill paid 10 euros
    X'019b5551261d768c80a3d047d742e141', -- Expense entry Id
    X'019b5648dcf47d7a8fbb2a414de4bcc6', -- Expense Id
    X'019b37f428ff7b9dbe10b91f7a0dec74', -- Group Id
    X'019b3752b7d87a208bb28d0a44a1f661', -- Payer Id (Bill)
    NULL, -- Status (active)
    1000,
    X'019b3752b7d87a208bb28d0a44a1f661', -- Author Id (Bill)
    '2025-12-01T08:00:50Z', -- Occured at
    '2025-12-02T10:00:50Z' -- Created at
),
( -- John and Bill shared expenses; old expense overwritten by the previous one
    X'019b5652e5ed7776b05b2a326da8471a', -- Expense entry Id
    X'019b5648dcf47d7a8fbb2a414de4bcc6', -- Expense Id
    X'019b37f428ff7b9dbe10b91f7a0dec74', -- Group Id
    X'019b3752b7d87a208bb28d0a44a1f661', -- Payer Id (Bill)
    X'019b5551261d768c80a3d047d742e141', -- Status (inactive)
    800,
    X'019b14ef290a70d9a2452a4723d9d44a', -- Author Id (John)
    '2025-12-01T08:00:50Z', -- Occured at
    '2025-12-01T16:00:50Z' -- Created at
),
( -- John and Bill shared expenses; another active expense
    X'019c165a4dfb73499269224f260c1fa7', -- Expense entry Id
    X'019c165a14f47e1a9cab336e6f5a513c', -- Expense Id
    X'019b37f428ff7b9dbe10b91f7a0dec74', -- Group Id
    X'019b3752b7d87a208bb28d0a44a1f661', -- Payer Id (Bill)
    NULL, -- Status (active)
    9000,
    X'019b3752b7d87a208bb28d0a44a1f661', -- Author Id (Bill)
    '2026-01-01T08:00:50Z', -- Occured at
    '2026-01-01T16:00:50Z' -- Created at
),
( -- Trip to Europe 2025; John updated amount to 5 euros
    X'019c15f2a7a67890baa3efec4299f288', -- Expense entry Id
    X'019c15f2c89d78cd84a2d945e2378772', -- Expense Id
    X'019b36d0b0ce72c7a1b46f44fcb55f22', -- Group Id
    X'019b14ef290a70d9a2452a4723d9d44a', -- Payer Id (John)
    NULL, -- Status (active)
    500,
    X'019b14ef290a70d9a2452a4723d9d44a', -- Author Id (John)
    '2025-12-01T08:00:50Z', -- Occured at
    '2025-12-03T16:00:50Z' -- Created at
),
( -- Trip to Europe 2025; John created expense for himself; overwritten by the previous one
    X'019c164cd98b718bb2d4ad9cd6e7f4d4', -- Expense entry Id
    X'019c15f2c89d78cd84a2d945e2378772', -- Expense Id
    X'019b36d0b0ce72c7a1b46f44fcb55f22', -- Group Id
    X'019b14ef290a70d9a2452a4723d9d44a', -- Payer Id (John)
    X'019c15f2a7a67890baa3efec4299f288', -- Status (inactive)
    500,
    X'019b14ef290a70d9a2452a4723d9d44a', -- Author Id (John)
    '2025-12-01T08:00:50Z', -- Occured at
    '2025-12-01T16:00:50Z' -- Created at
)
;


INSERT INTO expense_entry_participant (expense_entry_id, participant_id) VALUES
(
    X'019b5551261d768c80a3d047d742e141',
    X'019b14ef290a70d9a2452a4723d9d44a' -- John
),
(
    X'019b5652e5ed7776b05b2a326da8471a',
    X'019b14ef290a70d9a2452a4723d9d44a' -- John
),
(
    X'019c165a4dfb73499269224f260c1fa7',
    X'019b14ef290a70d9a2452a4723d9d44a' -- John
)
;
