use super::schema::contact;
use super::schema::contact::dsl::contact as contact_dal;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct Response<T> {
  pub status: i32,
  pub result: T,
}

// this is to insert users to database
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "contact"]
pub struct NewContact {
  pub first_name: String,
  pub last_name: String,
}

#[derive(Serialize, Queryable)]
pub struct Contact {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
}

impl Contact {
  pub fn get_all(conn: &PgConnection) -> Vec<Contact> {
    contact_dal
      .order(contact::id.desc())
      .load::<Contact>(conn)
      .expect("error!")
  }

  pub fn insert(new_contact: NewContact, conn: &PgConnection) -> bool {
    diesel::insert_into(contact::table)
      .values(&new_contact)
      .execute(conn)
      .is_ok()
  }

  pub fn get_by_id(id: i32, conn: &PgConnection) -> Vec<Contact> {
    contact_dal
      .filter(contact::id.eq(id))
      .load::<Contact>(conn)
      .expect("error!")
  }
}
