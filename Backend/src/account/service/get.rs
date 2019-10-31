use crate::account::material::account::Account;
use crate::account::tools::get::GetAccountInformation;

use rocket::response::content;
use rocket::State;
use serde_json::to_string;

#[get("/get/<id>")]
pub fn get(me: State<Account>, id: u32) -> content::Json<String>
{
  match me.get(id) {
    Ok(acc_info) => content::Json(to_string(&acc_info).unwrap()),
    Err(err_str) => content::Json(err_str)
  }
}