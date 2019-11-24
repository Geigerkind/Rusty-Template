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

use account::Account;

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
    account::transfer::api::api,
    account::transfer::token::create_token, account::transfer::token::get_tokens, account::transfer::token::delete_token,
    account::transfer::delete::request, account::transfer::delete::confirm,
    account::transfer::create::create, account::transfer::create::confirm, account::transfer::create::resend_confirm,
    account::transfer::get::get_account_information,
    account::transfer::forgot::receive_confirmation, account::transfer::forgot::send_confirmation,
    account::transfer::update::mail, account::transfer::update::password, account::transfer::update::nickname]);
  igniter.launch();
}