SELECT
    1
FROM
    GroupMembership
    JOIN Threads ON Threads.group = GroupMembership.groupid
    JOIN (
        (
            SELECT
                Originals.thread_id AS thread_id,
                Originals.post_id AS post_id,
            FROM
                Originals
        )
        UNION
        (
            SELECT
                Corrections.post_id AS post_id,
                Originals.thread_id AS thread_id,
            FROM
                Corrections
                JOIN Originals on Corrections.orig_id = Originals.post_id
        )
    ) p ON p.thread_id = Threads.id
    WHERE p.post_id = :post_id
    AND GroupMembership.userid = :user_id