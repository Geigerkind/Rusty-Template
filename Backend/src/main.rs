#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate mysql;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate lettre;
extern crate lettre_email;

pub mod util;
pub mod account;
pub mod database;

use account::material::account::Account;

#[allow(dead_code)]
pub struct Backend {
  data_acc: Account,
}

impl Backend {
}

fn main() {
  let mut igniter = rocket::ignite();
  let account = Account::new();
  account.init();
  let backend_obj = Backend {
    data_acc: account
  };
  igniter = igniter.manage(backend_obj);
  igniter = igniter.mount("/API/account/", routes![
    account::dto::delete::request, account::dto::delete::confirm,
    account::dto::create::create, account::dto::create::confirm, account::dto::create::resend_confirm,
    account::dto::get::get,
    account::dto::forgot::receive_confirmation, account::dto::forgot::send_confirmation,
    account::dto::update::mail, account::dto::update::password, account::dto::update::nickname]);
  igniter.launch();
}