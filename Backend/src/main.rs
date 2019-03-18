#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::response::content;

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



fn main() {
    let mut igniter = rocket::ignite();
    igniter = igniter.mount("/", routes![index, hi, echo]);
    igniter = igniter.mount("/foo", routes![bar]);
    igniter.launch();
}