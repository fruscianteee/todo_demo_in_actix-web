CREATE TABLE todos (
    id SERIAL PRIMARY KEY,
    test TEXT NOT NULL,
    complated BOOLEAN NOT NULL DEFAULT false
);
