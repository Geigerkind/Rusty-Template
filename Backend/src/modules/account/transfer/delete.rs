use rocket::State;

use crate::modules::account::guard::Authenticate;
use crate::modules::account::material::Account;
use crate::modules::account::tools::Delete;
use crate::modules::account::dto::Failure;

#[get("/delete/confirm/<id>", format="application/json")]
pub fn confirm(me: State<Account>, id: String) -> Result<(), Failure>
{
  me.confirm_delete(&id)
}

#[delete("/delete/request", format="application/json")]
pub fn request(me: State<Account>, auth: Authenticate) -> Result<(), Failure>
{
  me.issue_delete(auth.0)
}

