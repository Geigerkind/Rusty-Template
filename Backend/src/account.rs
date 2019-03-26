use rocket::response::content;
use crate::Backend;
use rocket::State;

pub trait Account {
    fn get_sample_account_db_fn(&self) -> i32;
}

impl Account for Backend {
    fn get_sample_account_db_fn(&self) -> i32
    {
        0
    }
}

#[get("/bar")]
pub fn foo(me: State<Backend>) -> content::Json<String> {
    content::Json(me.get_sample_account_db_fn().to_string())
}