-- Your SQL goes here
create table if not exists users(
    id serial primary key,
    first_name varchar not null,
    last_name varchar not null,
    email varchar not null unique
);