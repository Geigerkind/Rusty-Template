#![feature(proc_macro_hygiene, decl_macro)]
extern crate language;
extern crate mail;
#[macro_use]
extern crate mysql_connection;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate str_util;
extern crate time_util;
extern crate validator;

use rocket_contrib::json::Json;

use account::material::Account;

pub mod account;

#[allow(dead_code)]
pub struct Backend {
  account: Account,
}

impl Backend {
  fn new() -> Self
  {
    let account = Account::default();
    account.init();
    Backend {
      account
    }
  }
}

#[get("/")]
fn api_overview() -> Json<Vec<String>> {
  Json(vec!["/API/account/".to_string()])
}

fn main() {
  let mut igniter = rocket::ignite();
  let backend = Backend::new();
  igniter = igniter.manage(backend.account);
  igniter = igniter.mount("/API/", routes![api_overview]);
  igniter = igniter.mount("/API/account/", routes![
    account::service::api::api,
    account::service::token::create_token, account::service::token::get_tokens, account::service::token::delete_token,
    account::service::delete::request, account::service::delete::confirm,
    account::service::create::create, account::service::create::confirm, account::service::create::resend_confirm,
    account::service::get::get,
    account::service::forgot::receive_confirmation, account::service::forgot::send_confirmation,
    account::service::update::mail, account::service::update::password, account::service::update::nickname]);
  igniter.launch();
}