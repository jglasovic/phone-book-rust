CREATE TABLE phone
(
    id           SERIAL PRIMARY KEY,
    phone_number VARCHAR NOT NULL UNIQUE,
    contact_id   SERIAL REFERENCES contact(id) ON DELETE CASCADE
);