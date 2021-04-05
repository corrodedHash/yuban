SELECT
    Posts.id AS post_id,
    Posts.postdate AS postdate,
    SUBSTRING(Posts.post, 1, 10) AS ellipsis,
    Users.username AS username,
    Originals.langcode AS lang,
    Corr.corrsum AS corrections
FROM
    Posts
    JOIN Users ON Posts.userid = Users.id
    JOIN Originals ON Posts.id = Originals.post_id
    JOIN Threads ON Threads.id = Originals.thread_id
    JOIN Users threaduser ON Threads.owner_id = threaduser.id
    JOIN (
        SELECT
            Originals.post_id AS post_id,
            JSON_ARRAYAGG(corr_summary.corrsum) AS corrsum
        FROM
            Originals
            LEFT JOIN (
                SELECT
                    Corrections.orig_id AS orig_id,
                    JSON_OBJECT(
                        'id',
                        Corrections.post_id,
                        'username',
                        Users.username,
                        'postdate',
                        Posts.postdate
                    ) AS corrsum
                FROM
                    Corrections
                    JOIN Posts ON Posts.id = Corrections.post_id
                    JOIN Users ON Users.id = Posts.userid
            ) corr_summary ON Originals.post_id = corr_summary.orig_id
        GROUP BY
            Originals.post_id
    ) Corr ON Posts.id = Corr.post_id
WHERE
    Threads.id = :thread_id