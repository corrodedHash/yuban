-- https://jira.mariadb.org/browse/MDEV-23004
-- Cannot use JSON_ARRAYAGG, bugged in Ver 15.1 Distrib 10.5.9-MariaDB
SELECT Groups.id, Groups.groupname, CONCAT("[", GROUP_CONCAT(JSON_QUOTE(Users.username)), "]")
FROM Groups
LEFT JOIN GroupMembership ON Groups.id = GroupMembership.groupid
LEFT JOIN Users ON Users.id = GroupMembership.userid
GROUP BY Groups.groupname