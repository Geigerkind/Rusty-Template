#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate mysql;
#[macro_use] extern crate serde_derive;
extern crate rand;
extern crate lettre;
extern crate lettre_email;


use rocket::response::content;
use rocket::State;
use std::sync::RwLock;

pub mod account;
pub mod word;
pub mod mysqlconnection;
pub mod mail;

use mysqlconnection::MySQLConnection;
use account::Account;
use account::AccountData;
use word::Word;

pub struct Backend {
    count: RwLock<u8>,
    db_main: MySQLConnection,
    // WTB: Field values for traits!
    data_acc: AccountData,
}

impl Backend {
    pub fn increment(&self)
    {
        let mut data = self.count.write().unwrap();
        *data += 1;
    }
    pub fn get_count(&self) -> u8
    {
        let data = self.count.read().unwrap();
        *data
    }

    pub fn get_db_sample(&self) -> i32
    {
        let res: i32 = self.db_main.select_value("SELECT (1234)", &|row| {
            let val = mysql::from_row(row);
            val
        }).unwrap();
        res
    }
}

#[get("/")]
fn index() -> content::Json<&'static str> {
    content::Json("{\"text\": \"Hello World\"}")
}

#[get("/test")]
fn hi() -> content::Json<&'static str> {
    content::Json("{\"text\": \"Hello World 2\"}")
}

#[get("/")]
fn bar() -> content::Json<&'static str> {
    content::Json("{\"text\": \"Hello World 3\"}")
}

#[get("/echo/<name>")]
fn echo(name: String) -> content::Json<String> {
    let res = format!("{{\"text\": \"{}\"}}", name).to_string();
    content::Json(res)
}


#[get("/count")]
fn count(me: State<Backend>) -> content::Json<String> {
    me.increment();
    let data = me.get_count();
    content::Json(data.to_string())
}


#[get("/dbtest")]
fn dbtest(me: State<Backend>) -> content::Json<String>
{
    let res = me.get_db_sample();
    content::Json(res.to_string())
}


fn main() {
    let mut igniter = rocket::ignite();
    let backend_obj = Backend { 
        count: RwLock::new(0),
        db_main: MySQLConnection::new("main"),
        data_acc: AccountData::new()
    };
    Account::init(&backend_obj);
    Word::init(&backend_obj);
    igniter = igniter.manage(backend_obj);
    igniter = igniter.mount("/API/", routes![index, hi, echo, count, dbtest]);
    igniter = igniter.mount("/API/account/", routes![account::delete, account::create, account::get]);
    igniter.launch();
}