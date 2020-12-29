use super::contact::Contact;
use crate::schema::contact_email::{self, dsl::contact_email as email_dal};
use diesel::{pg::PgConnection, prelude::*};
use serde_derive::{Deserialize, Serialize};
use std::option::Option;

pub struct EmailDTO {
  pub id: i32,
  pub email: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "contact_email"]
pub struct NewEmail {
  pub email: String,
  pub contact_id: i32,
}

#[derive(Serialize, Queryable, Deserialize, Associations)]
#[belongs_to(Contact)]
#[table_name = "contact_email"]
pub struct Email {
  pub id: i32,
  pub email: String,
  pub contact_id: i32,
}

impl Email {
  pub fn get_all(conn: &PgConnection) -> Vec<Email> {
    email_dal
      .order(contact_email::id.desc())
      .load::<Email>(conn)
      .expect("error!")
  }

  pub fn insert(new_Email: NewEmail, conn: &PgConnection) -> bool {
    diesel::insert_into(contact_email::table)
      .values(&new_Email)
      .execute(conn)
      .is_ok()
  }

  pub fn get_by_contact_id(id: i32, conn: &PgConnection) -> Option<&Email> {
    let results = email_dal
      .filter(contact_email::contact_id.eq(id))
      .load::<Email>(conn)
      .expect("error!");

    results.first()
  }
  pub fn into_DTO(&self) -> EmailDTO {
    EmailDTO {
      email: self.email,
      id: self.id,
    }
  }
}
