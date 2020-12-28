CREATE TABLE contact_email
(
    id         SERIAL PRIMARY KEY,
    email      VARCHAR NOT NULL UNIQUE,
    contact_id SERIAL REFERENCES contact(id) ON DELETE CASCADE
);