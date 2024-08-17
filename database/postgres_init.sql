CREATE USER testuser with password '123';
CREATE DATABASE test;
GRANT ALL PRIVILEGES ON DATABASE test TO testuser;
\c test
GRANT ALL ON SCHEMA public TO testuser;