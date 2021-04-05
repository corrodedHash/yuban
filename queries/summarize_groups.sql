SELECT Groups.id, Groups.groupname, JSON_ARRAYAGG(Users.username)
FROM Groups
LEFT JOIN GroupMembership ON Groups.id = GroupMembership.groupid
LEFT JOIN Users ON Users.id = GroupMembership.userid
GROUP BY Groups.groupname