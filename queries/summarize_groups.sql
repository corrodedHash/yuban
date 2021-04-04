SELECT Groups.groupname, JSON_ARRAYAGG(Users.username)
FROM Groups
JOIN GroupMembership ON Groups.id = GroupMembership.groupid
JOIN Users ON Users.id = GroupMembership.userid
GROUP BY Groups.groupname