use rocket::response::content;
use crate::Backend;
use rocket::State;

pub trait Account {
    fn get_sample_account_db_fn(&self) -> i32;
}

impl Account for Backend {
    fn get_sample_account_db_fn(&self) -> i32
    {
        let res: i32 = self.conn.select_value("SELECT (1234)", &|row| {
            let val = mysql::from_row(row);
            val
        }).unwrap();
        res
    }
}

#[get("/bar")]
pub fn foo(me: State<Backend>) -> content::Json<String> {
    content::Json(me.get_sample_account_db_fn().to_string())
}