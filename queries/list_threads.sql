SELECT
    Threads.id,
    Users.username,
    Threads.opened_on,
    JSON_ARRAYAGG(Originals.langcode),
    JSON_ARRAYAGG(corr_counts.counts)
FROM
    Threads
    JOIN Originals ON Threads.id = Originals.thread_id
    LEFT JOIN (
        SELECT
            Corrections.orig_id,
            as orig_id,
            COUNT(Corrections.post_id) as counts
        FROM
            Corrections
        GROUP BY
            Corrections.orig_id
    ) corr_counts ON corr_counts.orig_id = Originals.post_id
    JOIN Users ON Users.id = Threads.creator
GROUP BY
    Threads.id,
    Threads.creator,
    Threads.opened_on
WHERE
    Threads.groupid = :groupid