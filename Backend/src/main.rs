#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate mysql;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

pub mod util;
pub mod account;
pub mod database;

use account::material::account::Account;

#[allow(dead_code)]
pub struct Backend {
  account: Account,
}

impl Backend {
  fn new() -> Self
  {
    let account = Account::new();
    account.init();
    Backend {
      account: account
    }
  }
}

fn main() {
  let mut igniter = rocket::ignite();
  igniter = igniter.manage(Backend::new());
  igniter = igniter.mount("/API/account/", routes![
    account::dto::delete::request, account::dto::delete::confirm,
    account::dto::create::create, account::dto::create::confirm, account::dto::create::resend_confirm,
    account::dto::get::get,
    account::dto::forgot::receive_confirmation, account::dto::forgot::send_confirmation,
    account::dto::update::mail, account::dto::update::password, account::dto::update::nickname]);
  igniter.launch();
}