use rocket::State;

use crate::account::guard::Authenticate;
use crate::account::material::Account;
use crate::account::tools::Delete;
use crate::account::dto::Failure;

#[get("/delete/confirm/<id>")]
pub fn confirm(me: State<Account>, id: String) -> Result<(), Failure>
{
  me.confirm_delete(&id)
}

#[delete("/delete/request")]
pub fn request(me: State<Account>, auth: Authenticate) -> Result<(), Failure>
{
  me.issue_delete(auth.0)
}

