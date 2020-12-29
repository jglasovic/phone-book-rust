use super::email::EmailDTO;
use super::phone::PhoneDTO;
use super::tag::TagDTO;
use crate::schema::contact::{self, dsl::contact as contact_dal};
use diesel::{pg::PgConnection, prelude::*};
use serde_derive::{Deserialize, Serialize};
use std::option::Option;

pub struct ContactDTO {
  pub id: i32,
  pub first_name: String,
  pub last_name: String,
  pub emails: Vec<EmailDTO>,
  pub phone_numbers: Vec<PhoneDTO>,
  pub tags: Vec<TagDTO>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "contact"]
pub struct NewContact {
  pub first_name: String,
  pub last_name: String,
  // extend to support emails, numbers, tags
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

  pub fn get_by_id(id: i32, conn: &PgConnection) -> Option<&Contact> {
    let results = contact_dal
      .filter(contact::id.eq(id))
      .load::<Contact>(conn)
      .expect("error!");

    results.first()
  }
  pub fn into_DTO(
    &self,
    emails: Vec<EmailDTO>,
    phone_numbers: Vec<PhoneDTO>,
    tags: Vec<TagDTO>,
  ) -> ContactDTO {
    ContactDTO {
      id: self.id,
      first_name: self.first_name,
      last_name: self.last_name,
      emails,
      phone_numbers,
      tags,
    }
  }
}
