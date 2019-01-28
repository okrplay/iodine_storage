-- Your SQL goes here
create table users  (
    userid bigserial primary key,
    username varchar not null,
    fullname varchar not null,
    password varchar not null,
    generation varchar not null
);