use crate::Backend;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

pub struct Member {
    id: u32,
    mail: String,
    password: String
}

pub trait Account {
    fn init(&self);

    fn create(&self, params: &PostCreate) -> bool;
    fn delete(&self, params: &PostDelete) -> bool;
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

    // TODO: Do hashing, checking if it exists etc.
    fn create(&self, params: &PostCreate) -> bool
    {
        self.db_main.execute_wparams("INSERT IGNORE INTO member (`mail`, `password`) VALUES (:mail, :pass)", params!(
            "mail" => params.mail.to_owned(),
            "pass" => params.password.to_owned()
        ))
    }

    fn delete(&self, params: &PostDelete) -> bool
    {
        self.db_main.execute_wparams("DELETE FROM member WHERE id = :id", params!(
            "id" => params.id
        ))
    }
}

/**
* Rocket request handling
**/

#[derive(Deserialize)]
pub struct PostCreate{
    mail: String,
    password: String
}
#[post("/create", data = "<params>")]
pub fn create(me: State<Backend>, params: Json<PostCreate>) -> content::Json<String> {
    content::Json(me.create(&params).to_string())
}

// TODO: Add validation
#[derive(Deserialize)]
pub struct PostDelete{
    id: u32
}
#[post("/delete", data = "<params>")]
pub fn delete(me: State<Backend>, params: Json<PostDelete>) -> content::Json<String> {
    content::Json(me.delete(&params).to_string())
}