-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    comment VARCHAR NULL,
    active BOOLEAN NOT NULL DEFAULT 'f',
    pass_hash VARCHAR NOT NULL
);

CREATE TABLE messages (
     id SERIAL PRIMARY KEY,
     header VARCHAR NOT NULL,
     message VARCHAR NOT NULL,
     sender_user_id INT,
     receiver_user_id INT,
        CONSTRAINT fk_sender FOREIGN KEY(sender_user_id) REFERENCES users(id),
        CONSTRAINT fk_receiver FOREIGN KEY(receiver_user_id) REFERENCES users(id)
);