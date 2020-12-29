use super::contact::*;
use super::db::Conn as DbConn;
use rocket_contrib::json::Json;
use serde_derive::Deserialize;
use serde_json::Value;

// #[derive(Deserialize)]
pub struct Response<T> {
  pub status: i32,
  pub result: T,
}

#[get("/contact", format = "application/json")]
pub fn get_all(conn: DbConn) -> Json<Value> {
  let contacts = Contact::get_all(&conn);
  let response = Response::<Vec<Contact>> {
    status: 200,
    result: contacts,
  };
  Json(json!(response))
}

#[post("/contact", format = "application/json", data = "<new_contact>")]
pub fn add_new(conn: DbConn, new_contact: Json<NewContact>) -> Json<Value> {
  Json(json!({
      "status": if Contact::insert(new_contact.into_inner(), &conn) == true { 200 } else { 500 },
      "result": Contact::get_all(&conn).first(),
  }))
}

#[get("/contact/<id>", format = "application/json")]
pub fn find_single(conn: DbConn, id: String) -> Json<Value> {
  let user_id: i32 = id.trim().parse().expect("error");
  Json(json!({
      "status": 200,
      "result": Contact::get_by_id(user_id, &conn),
  }))
}
