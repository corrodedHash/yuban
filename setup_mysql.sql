CREATE DATABASE yuban;
USE yuban;

CREATE TABLE Users (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(30) NOT NULL UNIQUE,
    passwordHash BINARY(32) NOT NULL,
    salt BINARY(16) NOT NULL
);

CREATE TABLE Tokens (
    userid INT UNSIGNED NOT NULL UNIQUE,
    token BINARY(32) NOT NULL,
    issuedate TIMESTAMP NOT NULL,
    FOREIGN KEY (userid)
        REFERENCES Users(id)
        ON DELETE CASCADE
);

CREATE TABLE Threads (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    owner_id INT UNSIGNED NOT NULL,
    opened_on TIMESTAMP NOT NULL,

    FOREIGN KEY (owner_id)
        REFERENCES Users(id)
        ON DELETE CASCADE
);

CREATE TABLE Posts (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    userid INT UNSIGNED NOT NULL,
    postdate TIMESTAMP NOT NULL,
    post MEDIUMTEXT NOT NULL,

    FOREIGN KEY (userid)
        REFERENCES Users(id)
        ON DELETE CASCADE
);

CREATE TABLE Originals (
    thread_id INT UNSIGNED NOT NULL,
    post_id INT UNSIGNED NOT NULL,
    langcode VARCHAR(2) NOT NULL,

    FOREIGN KEY (thread_id)
        REFERENCES Threads(id)
        ON DELETE CASCADE,
    FOREIGN KEY (post_id)
        REFERENCES Posts(id)
        ON DELETE CASCADE
);

CREATE TABLE Corrections (
    orig_id INT UNSIGNED NOT NULL,
    post_id INT UNSIGNED NOT NULL,

    FOREIGN KEY (orig_id)
        REFERENCES Originals(post_id)
        ON DELETE CASCADE,
    FOREIGN KEY (post_id)
        REFERENCES Posts(id)
        ON DELETE CASCADE
);

CREATE USER 'yubanmanager'@'%' IDENTIFIED WITH mysql_native_password BY 'PajqMDXIloNcwuxG27udp3gy4EBi';

GRANT DELETE ON yuban.* TO 'yubanmanager'@'%';
GRANT INSERT ON yuban.* TO 'yubanmanager'@'%';
GRANT SELECT ON yuban.* TO 'yubanmanager'@'%';
GRANT UPDATE ON yuban.* TO 'yubanmanager'@'%';

DROP USER 'root'@'%';
