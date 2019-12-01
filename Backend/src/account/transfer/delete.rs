use rocket::State;

use crate::account::guard::authenticate::Authenticate;
use crate::account::material::Account;
use crate::account::tools::Delete;

#[get("/delete/confirm/<id>")]
pub fn confirm(me: State<Account>, id: String) -> Result<(), String>
{
  me.confirm_delete(&id)
}

#[delete("/delete/request")]
pub fn request(me: State<Account>, auth: Authenticate) -> Result<(), String>
{
  me.issue_delete(auth.0)
}

