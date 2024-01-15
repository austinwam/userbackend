CREATE TABLE users (
    userid SERIAL PRIMARY KEY,
    username VARCHAR ( 70 ) NOT NULL,
    email VARCHAR ( 80 ) NOT NULL,
    phone VARCHAR ( 20 ) NOT NULL,
    status VARCHAR ( 20 ),
    paid int DEFAULT 0,
    unpaid int DEFAULT 0,
    amount int DEFAULT 0,
    role json,
    password VARCHAR ( 90 ),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

