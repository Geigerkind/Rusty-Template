use rocket::response::content;
use crate::Backend;
use rocket::State;

pub trait Account {
    fn get_sample_account_db_fn(&self) -> i32;
}

impl Account for Backend {
    fn get_sample_account_db_fn(&self) -> i32
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

#[get("/bar")]
pub fn foo(me: State<Backend>) -> content::Json<String> {
    content::Json(me.get_sample_account_db_fn().to_string())
}