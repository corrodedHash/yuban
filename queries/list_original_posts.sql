SELECT
    id,
    opened_on,
    user,
    JSON_ARRAYAGG(post)
FROM
    (
        SELECT
            Threads.id AS id,
            Threads.opened_on AS opened_on,
            threaduser.id as user,
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
                Corr.corr_ids
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
                    JSON_ARRAYAGG(Corrections.post_id) AS corr_ids
                FROM
                    Originals
                    LEFT JOIN Corrections ON Originals.post_id = Corrections.orig_id
                GROUP BY
                    Originals.post_id
            ) Corr ON Posts.id = Corr.post_id
    ) threaded_posts
GROUP BY
    id,
    opened_on,
    user
ORDER BY
    opened_on DESC