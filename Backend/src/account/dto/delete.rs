use crate::Backend;
use crate::account::api::Account;
use crate::account::api::ValidationPair;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;

#[get("/delete/confirm/<id>")]
pub fn confirm(me: State<Backend>, id: String) -> content::Json<String>
{
    content::Json(me.confirm_delete(&id).to_string())
}

#[delete("/delete/send", data = "<params>")]
pub fn request(me: State<Backend>, params: Json<ValidationPair>) -> content::Json<String>
{
    content::Json(me.issue_delete(&params).to_string())
}

