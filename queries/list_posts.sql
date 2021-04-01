SELECT
    Threads.id AS id,
    Threads.opened_on AS opened_on,
    threaduser.username as user,
    JSON_OBJECT(
        'id',
        Posts.id,
        'date',
        Posts.postdate,
        'ellipsis',
        SUBSTRING(Posts.post, 1, 10),
        'user',
        Users.username,
        'lang',
        Originals.langcode,
        'corrections',
        Corr.corrsum
    ) AS post
FROM
    Posts
    INNER JOIN Users ON Posts.userid = Users.id
    INNER JOIN Originals ON Posts.id = Originals.post_id
    INNER JOIN Threads ON Threads.id = Originals.thread_id
    INNER JOIN Users threaduser ON Threads.owner_id = threaduser.id
    INNER JOIN (
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
                    INNER JOIN Posts ON Posts.id = Corrections.post_id
                    INNER JOIN Users ON Users.id = Posts.userid
            ) corr_summary ON Originals.post_id = corr_summary.orig_id
        GROUP BY
            Originals.post_id
    ) Corr ON Posts.id = Corr.post_id
WHERE
    Threads.id = :thread_id