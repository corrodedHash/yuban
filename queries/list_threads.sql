SELECT
    Threads.id,
    Users.username,
    Threads.opened_on,
    COALESCE(
        CONCAT(
            "[",
            GROUP_CONCAT(translations.langs),
            "]"
        ),
        "[]"
    ) AS translation_languages
FROM
    Threads
    JOIN Users ON Users.id = Threads.creator
    LEFT JOIN (
        SELECT
            Originals.thread_id,
            JSON_OBJECT(
                'lang',
                Originals.langcode,
                'count',
                corr_counts.counts
            ) AS langs
        FROM
            Originals
            LEFT JOIN(
                SELECT
                    Corrections.orig_id AS orig_id,
                    COUNT(Corrections.post_id) AS counts
                FROM
                    Corrections
                GROUP BY
                    Corrections.orig_id
            ) corr_counts ON corr_counts.orig_id = Originals.post_id
    ) translations ON translations.thread_id = Threads.id
WHERE
    Threads.groupid = :groupid
GROUP BY
    Threads.id,
    Threads.creator,
    Threads.opened_on
ORDER BY
    Threads.opened_on DESC