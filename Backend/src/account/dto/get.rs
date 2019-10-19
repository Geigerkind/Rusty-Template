use crate::Backend;
use crate::account::tools::account::Account;

use rocket::response::content;
use rocket::State;
use serde_json::to_string;

#[get("/get/<id>")]
pub fn get(me: State<Backend>, id: u32) -> content::Json<String>
{
  match me.get(id) {
    Some(acc_info) => content::Json(to_string(&acc_info).unwrap()),
    None => content::Json("Error?!".to_string()) // 404?
  }
}