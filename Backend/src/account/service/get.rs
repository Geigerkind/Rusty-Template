use crate::account::material::account::Account;
use crate::account::tools::get::GetAccountInformation;
use crate::account::domainvalue::account_information::AccountInformation;

use rocket::State;
use rocket_contrib::json::Json;

#[openapi]
#[get("/get/<id>")]
pub fn get(me: State<Account>, id: u32) -> Result<Json<AccountInformation>, String>
{
  match me.get(id) {
    Ok(acc_info) => Ok(Json(acc_info)),
    Err(err_str) => Err(err_str)
  }
}