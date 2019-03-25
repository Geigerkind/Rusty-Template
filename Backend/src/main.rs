#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate mysql;

use rocket::response::content;
use rocket::State;
use std::sync::Mutex;

pub mod account;

/*
#[database("mysql")]
struct DBConn(mysql::Conn);
impl DBConn {
    pub fn get_sample(conn: &mysql::Conn) -> &u8 
    {
        let res: Vec<u8> = conn.prep_exec("SELECT (1234)", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let val = mysql::from_row(row);
                val
            }).collect()
        }).unwrap();
        res.first().unwrap()
    }
}*/

struct Backend {
    count: Mutex<u8>,
    conn: Mutex<mysql::Pool>
}

impl Backend {
    pub fn increment(&self)
    {
        let mut data = self.count.lock().unwrap();
        *data += 1;
    }
    pub fn get_count(&self) -> u8
    {
        let data = self.count.lock().unwrap();
        *data
    }

    pub fn get_db_sample(&self) -> i32
    {
        let conn = self.conn.lock().unwrap();
        let res: Vec<i32> = conn.prep_exec("SELECT (1234)", ())
        .map(|result| {
            result.map(|x| x.unwrap()).map(|row| {
                let val = mysql::from_row(row);
                val
            }).collect()
        }).unwrap();
        *res.first().unwrap()
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
    igniter = igniter.manage(Backend { 
        count: Mutex::new(0),
        conn: Mutex::new(mysql::Pool::new("mysql://root@127.0.0.1/test").unwrap())
    });
    igniter = igniter.mount("/API/", routes![index, hi, echo, count, account::foo, dbtest]);
    igniter = igniter.mount("/API/foo", routes![bar]);
    igniter.launch();
}