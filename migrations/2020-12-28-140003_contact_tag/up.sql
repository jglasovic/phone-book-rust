CREATE TABLE contact_tag (
  id SERIAL PRIMARY KEY,
  contact_id SERIAL REFERENCES contact(id) ON DELETE CASCADE,
  tag_id SERIAL REFERENCES tag(id) ON DELETE CASCADE,
  CONSTRAINT contact_tag_cons UNIQUE (contact_id, tag_id)
)