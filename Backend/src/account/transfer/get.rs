use rocket::State;
use rocket_contrib::json::Json;

use crate::account::domain_value::AccountInformation;
use crate::account::material::Account;
use crate::account::tools::GetAccountInformation;
use crate::account::guard::authenticate::Authenticate;

#[get("/get")]
pub fn get_account_information(me: State<Account>, auth: Authenticate) -> Result<Json<AccountInformation>, String>
{
  me.get(auth.0)
    .and_then(|acc_info| Ok(Json(acc_info)))
}