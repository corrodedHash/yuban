SELECT
    Groups.id as groupid,
    Groups.groupname as groupname,
    COUNT(Threads.id) as threadcount
FROM
    Groups
    JOIN Threads ON Groups.id = Threads.groupid
    JOIN GroupMembership ON GroupMembership.groupid = Groups.id
    JOIN Users ON GroupMembership.userid = Users.id
WHERE
    Users.username = :username
GROUP BY
    Groups.id,
    Groups.groupname