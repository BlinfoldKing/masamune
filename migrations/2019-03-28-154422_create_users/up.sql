-- Your SQL goes here
CREATE TABLE users (
id VARCHAR(6) PRIMARY KEY,
username VARCHAR(16),
email VARCHAR(16),
firstname VARCHAR(255),
lastname VARCHAR(255),
password VARCHAR(255)
);

INSERT INTO users VALUES (
'000000',
'testUser',
'test@user.com',
'test',
'user',
'test_user'
);

