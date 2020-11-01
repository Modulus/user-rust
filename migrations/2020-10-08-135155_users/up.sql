-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    comment VARCHAR NULL,
    active BOOLEAN NOT NULL DEFAULT 'f',
    pass_hash VARCHAR NOT NULL,
    UNIQUE(name)
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

CREATE TABLE friends (
    user_id INT NOT NULL,
    friend_id INT NOT NULL,
    PRIMARY KEY (user_id, friend_id),
        CONSTRAINT fk_user FOREIGN KEY(user_id) REFERENCES USERS(id),
        CONSTRAINT fk_friend FOREIGN KEY(friend_id) REFERENCES USERS(id)
);