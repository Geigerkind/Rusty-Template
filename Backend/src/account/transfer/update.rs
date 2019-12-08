use rocket::State;
use rocket_contrib::json::Json;

use crate::account::domain_value::AccountInformation;
use crate::account::dto::Failure;
use crate::account::guard::Authenticate;
use crate::account::material::{Account, APIToken};
use crate::account::tools::Update;

#[post("/update/password", data = "<content>")]
pub fn password(me: State<Account>, auth: Authenticate, content: String) -> Result<Json<APIToken>, Failure> {
  me.change_password(&content, auth.0)
    .and_then(|api_token| Ok(Json(api_token)))
}

#[post("/update/nickname", data = "<content>")]
pub fn nickname(me: State<Account>, auth: Authenticate, content: String) -> Result<Json<AccountInformation>, Failure> {
  me.change_name(&content, auth.0)
    .and_then(|acc_info| Ok(Json(acc_info)))
}

#[post("/update/mail/request", data = "<content>")]
pub fn request_mail(me: State<Account>, auth: Authenticate, content: String) -> Result<(), Failure> {
  me.request_change_mail(&content, auth.0)
}

#[get("/update/mail/confirm/<id>")]
pub fn confirm_mail(me: State<Account>, id: String) -> Result<Json<APIToken>, Failure> {
  me.confirm_change_mail(&id)
    .and_then(|api_token| Ok(Json(api_token)))
}