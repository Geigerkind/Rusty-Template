use rocket::response::content;
use crate::Backend;
use rocket::State;

pub struct Member {
    id: u32,
    mail: String,
    password: String
}

pub trait Account {
    fn init(&self);

    fn get_sample_account_db_fn(&self) -> i32;
}

impl Account for Backend {
    fn init(&self)
    {
        let mut vec = self.member.write().unwrap();
        *vec = self.db_main.select("SELECT id, mail, password FROM member", &|row|{
            let (id, mail, pass) = mysql::from_row(row);
            Member {
                id: id,
                mail: mail,
                password: pass
            }
        });
    }

    fn get_sample_account_db_fn(&self) -> i32
    {
        let res: i32 = self.db_main.select_value("SELECT (1234)", &|row| {
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