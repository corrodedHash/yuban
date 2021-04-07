SELECT
    extra_info.thread_id,
    Posts.id,
    Posts.postdate,
    Users.username,
    extra_info.langcode,
    extra_info.correction_for,
    Posts.post
FROM
    Posts
    INNER JOIN Users ON Posts.userid = Users.id
    INNER JOIN (
        SELECT
            Originals.post_id AS id,
            Originals.thread_id AS thread_id,
            Originals.langcode AS langcode,
            NULL AS correction_for
        FROM
            Originals
        UNION
        SELECT
            Corrections.post_id AS id,
            Originals.thread_id AS thread_id,
            Originals.langcode AS langcode,
            Corrections.orig_id AS correction_for
        FROM
            Corrections
            INNER JOIN Originals on Corrections.orig_id = Originals.post_id
    ) extra_info ON extra_info.id = Posts.id
WHERE
    Posts.id = :postid;