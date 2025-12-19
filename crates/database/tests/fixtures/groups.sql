INSERT INTO coin_group (id, name, owner_id, created_at) VALUES
(X'019b36d0b0ce72c7a1b46f44fcb55f22', 'Trip to Europe 2025', X'019b14ef290a70d9a2452a4723d9d44a', '2025-08-14T23:00:00Z'),
(X'019b37f428ff7b9dbe10b91f7a0dec74', 'John and Bill shared expenses', X'019b14ef290a70d9a2452a4723d9d44a', '2025-10-23T09:00:50Z')
;


INSERT INTO coin_group_member (coin_group_id, member_id) VALUES
(X'019b37f428ff7b9dbe10b91f7a0dec74', X'019b3752b7d87a208bb28d0a44a1f661')
;
