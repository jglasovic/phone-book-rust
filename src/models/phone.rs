use super::contact::Contact;
use crate::schema::phone::{self, dsl::phone as phone_dal};
use diesel::{pg::PgConnection, prelude::*};
use serde_derive::{Deserialize, Serialize};
use std::option::Option;

pub struct PhoneDTO {
  pub id: i32,
  pub phone_number: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "phone"]
pub struct NewPhone {
  pub phone_number: String,
  pub contact_id: i32,
}

#[derive(Serialize, Queryable, Deserialize, Associations)]
#[belongs_to(Contact)]
#[table_name = "phone"]
pub struct Phone {
  pub id: i32,
  pub phone_number: String,
  pub contact_id: i32,
}

impl Phone {
  pub fn get_all(conn: &PgConnection) -> Vec<Phone> {
    phone_dal
      .order(phone::id.desc())
      .load::<Phone>(conn)
      .expect("error!")
  }

  pub fn insert(new_Phone: NewPhone, conn: &PgConnection) -> bool {
    diesel::insert_into(phone::table)
      .values(&new_Phone)
      .execute(conn)
      .is_ok()
  }

  pub fn get_by_contact_id(id: i32, conn: &PgConnection) -> Option<&Phone> {
    let results = phone_dal
      .filter(phone::contact_id.eq(id))
      .load::<Phone>(conn)
      .expect("error!");

    results.first()
  }
  pub fn into_DTO(&self) -> PhoneDTO {
    PhoneDTO {
      phone_number: self.phone_number,
      id: self.id,
    }
  }
}
