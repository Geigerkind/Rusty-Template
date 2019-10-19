#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate mysql;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;
extern crate lettre;
extern crate lettre_email;
extern crate regex;

pub mod util;
pub mod account;
pub mod mysqlconnection;

use mysqlconnection::MySQLConnection;
use account::tools::account::Account;
use account::material::account_data::AccountData;

pub struct Backend {
  db_main: MySQLConnection,
  // WTB: Field values for traits!
  data_acc: AccountData,
}

impl Backend {
}

fn main() {
  let mut igniter = rocket::ignite();
  let backend_obj = Backend {
    db_main: MySQLConnection::new("main"),
    data_acc: AccountData::new()
  };
  Account::init(&backend_obj);
  igniter = igniter.manage(backend_obj);
  igniter = igniter.mount("/API/account/", routes![
    account::dto::delete::request, account::dto::delete::confirm,
    account::dto::create::create, account::dto::create::confirm, account::dto::create::resend_confirm,
    account::dto::get::get,
    account::dto::forgot::receive_confirmation, account::dto::forgot::send_confirmation,
    account::dto::update::mail, account::dto::update::password, account::dto::update::nickname]);
  igniter.launch();
}