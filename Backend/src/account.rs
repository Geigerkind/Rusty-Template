use crate::Backend;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json::to_string;

pub struct Member {
    id: u32,
    mail: String,
    password: String,
    xp: u32
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
    fn get(&self, id: u32) -> Option<AccountInformation>;
}

impl Account for Backend {
    fn init(&self)
    {
        let mut member = self.member.write().unwrap();

        // We are a little wasteful here because we do not insert it directly but rather create a vector first and then copy it over
        for entry in self.db_main.select("SELECT id, mail, password, xp FROM member", &|row|{
            let (id, mail, pass, xp) = mysql::from_row(row);
            Member {
                id: id,
                mail: mail,
                password: pass,
                xp: xp
            }
        }) {
            member.insert(entry.id, entry);
        }
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

        if self.db_main.execute_wparams("INSERT IGNORE INTO member (`mail`, `password`) VALUES (:mail, :pass)", params!(
            "mail" => params.mail.to_owned(),
            "pass" => params.password.to_owned())
        ) {
            let mut member = self.member.write().unwrap();
            let id = self.db_main.select_wparams_value("SELECT id FROM member WHERE LOWER(mail) = :mail", &|row|{
                let res = mysql::from_row(row);
                res
            }, params!(
                "mail" => params.mail.to_owned().to_lowercase()
            )).unwrap();
            member.insert(id, Member {
                id: id,
                mail: params.mail.to_owned(),
                password: params.password.to_owned(),
                xp: 0
            });
        }
        true
    }

    fn delete(&self, params: &PostDeleteMember) -> bool
    {
        self.db_main.execute_wparams("DELETE FROM member WHERE id = :id", params!(
            "id" => params.id
        ))
    }

    fn get(&self, id: u32) -> Option<AccountInformation>
    {
        let member = self.member.read().unwrap();
        match member.get(&id) {
            Some(entry) => Some(AccountInformation {
                mail: entry.mail.clone(),
                xp: entry.xp
            }),
            None => None
        }
    }
}

/**
* Rocket request handling
**/

#[get("/get/<id>")]
pub fn get(me: State<Backend>, id: u32) -> content::Json<String>
{
    match me.get(id) {
        Some(acc_info) => content::Json(to_string(&acc_info).unwrap()),
        None => content::Json("Error?!".to_string())
    }
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