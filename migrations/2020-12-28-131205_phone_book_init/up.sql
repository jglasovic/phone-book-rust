CREATE TABLE contact
(
    id          SERIAL PRIMARY KEY,
    first_name  VARCHAR NOT NULL,
    last_name   VARCHAR NOT NULL
);

CREATE TABLE contact_email
(
    id         SERIAL PRIMARY KEY,
    email      VARCHAR NOT NULL UNIQUE,
    contact_id SERIAL REFERENCES contact(id) ON DELETE CASCADE
);

CREATE TABLE phone
(
    id           SERIAL PRIMARY KEY,
    phone_number VARCHAR NOT NULL UNIQUE,
    contact_id   SERIAL REFERENCES contact(id) ON DELETE CASCADE
);

CREATE TABLE tag
(
    id      SERIAL PRIMARY KEY,
    name    VARCHAR NOT NULL UNIQUE
);

CREATE TABLE contact_tag 
(
    id          SERIAL PRIMARY KEY,
    contact_id  SERIAL REFERENCES contact(id) ON DELETE CASCADE,
    tag_id      SERIAL REFERENCES tag(id) ON DELETE CASCADE,
    CONSTRAINT contact_tag_cons UNIQUE (contact_id, tag_id)
)