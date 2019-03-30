use crate::Backend;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json::to_string;

pub struct Member {
    id: u32,
    mail: String,
    password: String
}

#[derive(Serialize)]
pub struct AccountInformation {
    mail: String,
    xp: u32
}

pub trait Account {
    fn init(&self);

    fn create(&self, params: &PostCreateMember) -> bool;
    fn delete(&self, params: &PostDeleteMember) -> bool;
    fn get(&self, id: u32) -> AccountInformation;
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

    // TODO: Do hashing, checking if it exists, add it to existing structure, etc.
    fn create(&self, params: &PostCreateMember) -> bool
    {
        // Double spending check
        // We dont validate throguh the internal data structure because we may have race conditions
        if self.db_main.exists_wparams("SELECT id FROM member WHERE LOWER(mail) = :mail LIMIT 1", params!("mail" => params.mail.to_owned().to_lowercase())) 
        {
            return false;
        }

        self.db_main.execute_wparams("INSERT IGNORE INTO member (`mail`, `password`) VALUES (:mail, :pass)", params!(
            "mail" => params.mail.to_owned(),
            "pass" => params.password.to_owned()
        ))
    }

    fn delete(&self, params: &PostDeleteMember) -> bool
    {
        self.db_main.execute_wparams("DELETE FROM member WHERE id = :id", params!(
            "id" => params.id
        ))
    }

    fn get(&self, id: u32) -> AccountInformation
    {
        self.db_main.select_wparams_value("SELECT mail, xp FROM member WHERE id = :id", &|row| {
            let (mail, xp) = mysql::from_row(row);
            AccountInformation {
                mail: mail,
                xp: xp
            }
        }, params!(
            "id" => id
        )).unwrap()
    }
}

/**
* Rocket request handling
**/

#[get("/get/<id>")]
pub fn get(me: State<Backend>, id: u32) -> content::Json<String>
{
    content::Json(to_string(&me.get(id)).unwrap())
}

#[derive(Deserialize)]
pub struct PostCreateMember{
    mail: String,
    password: String
}
#[post("/create", data = "<params>")]
pub fn create(me: State<Backend>, params: Json<PostCreateMember>) -> content::Json<String> {
    content::Json(me.create(&params).to_string())
}

// TODO: Add validation
#[derive(Deserialize)]
pub struct PostDeleteMember{
    id: u32
}
#[delete("/delete", data = "<params>")]
pub fn delete(me: State<Backend>, params: Json<PostDeleteMember>) -> content::Json<String> {
    content::Json(me.delete(&params).to_string())
}