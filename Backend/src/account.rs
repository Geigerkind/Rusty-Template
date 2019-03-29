use rocket::response::content;
use crate::Backend;
use rocket::State;
use rocket::request::Form;

pub struct Member {
    id: u32,
    mail: String,
    password: String
}

pub trait Account {
    fn init(&self);

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

    fn delete(&self, params: &PostDelete) -> bool
    {
        self.db_main.execute_wparams("DELETE FROM member WHERE id = :id", params!(
            "id" => params.id
        ))
    }
}

// TODO: Add validation
#[derive(FromForm)]
pub struct PostDelete{
    id: u32
}
#[post("/delete", data = "<params>")]
pub fn delete(me: State<Backend>, params: Form<PostDelete>) -> content::Json<String> {
    content::Json(me.delete(&params).to_string())
}