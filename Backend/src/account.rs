use crate::Backend;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json::to_string;
use sha3::{Digest, Sha3_512};
use rand;

pub struct Member {
    id: u32,
    mail: String,
    password: String,
    salt: String,
    xp: u32,
    hash_prio: Vec<u8>,
    hash_val: Vec<String>
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
    fn login(&self, params: &PostLogin) -> Option<String>;
}

impl Account for Backend {
    fn init(&self)
    {
        let mut hash_to_member = self.hash_to_member.write().unwrap();
        let mut member = self.member.write().unwrap();

        // We are a little wasteful here because we do not insert it directly but rather create a vector first and then copy it over
        for entry in self.db_main.select("SELECT id, mail, password, salt, xp, val_hash1, val_prio1, val_hash2, val_prio2, val_hash3, val_prio3 FROM member", &|row|{
            let (id, mail, pass, salt, xp, val_hash1, val_prio1, val_hash2, val_prio2, val_hash3, val_prio3) = mysql::from_row(row);
            Member {
                id: id,
                mail: mail,
                password: pass,
                salt: salt,
                xp: xp,
                hash_prio: vec![val_prio1, val_prio2, val_prio3],
                hash_val: vec![val_hash1, val_hash2, val_hash3]
            }
        }) {
            // Chance should be fairly low that we a havea duplicate key
            for i in 0..2 {
                if entry.hash_val[i] != "none" {
                    hash_to_member.insert(entry.hash_val[i].clone(), entry.id);
                }
            }
            member.insert(entry.id, entry);
        }
    }

    // TODO: Do hashing etc.
    // TODO: Send validation mail
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
                password: params.password.to_owned(), // TODO: hash it
                salt: "".to_string(), // TODO
                xp: 0,
                hash_prio: vec![2,2,2],
                hash_val: vec!["none".to_string(), "none".to_string(), "none".to_string()]
            });
        }
        true
    }

    // TODO: Remove from hashmap
    // TODO: Validation
    // We might consider to send a mail first!
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

    fn login(&self, params: &PostLogin) -> Option<String>
    {
        // Do not change the order else we might end up in a dead lock!
        let mut hash_to_member = self.hash_to_member.write().unwrap();
        let mut member = self.member.write().unwrap();

        let lower_mail = params.mail.to_lowercase();
        let mut entry_key: u32 = 0;
        for (id, entry) in &(*member) {
            if entry.mail.to_lowercase() != lower_mail { continue; }
            let mut hasher = Sha3_512::new();
            let mut pw_hash_input: String = params.password.clone();
            pw_hash_input.push_str(&entry.salt);
            hasher.input(pw_hash_input);
            if entry.password != std::str::from_utf8(&hasher.result()).unwrap().to_string() { break; } // Password is wrong
            entry_key = *id;
            break
        }
        if entry_key == 0 { return None; }
        let entry = member.get_mut(&entry_key).unwrap();
            
        // Generate a 128 bit salt for our validation hash
        let salt: String = (0..15).map(|_| rand::random::<u8>() as char).collect();
        let mut val_hash = Sha3_512::new();
        let mut hash_input: String = entry.mail.clone();
        hash_input.push_str(&params.password);
        hash_input.push_str(&salt);
        val_hash.input(hash_input);
        let hash: String = std::str::from_utf8(&val_hash.result()).unwrap().to_string();

        // Replace by using the Least recently used strategy
        for i in 0..2 {
            if entry.hash_prio[i] >= 2 {
                // Adjusting prios
                entry.hash_prio[i] = 0;
                entry.hash_prio[(i+1)%3] += 1;
                entry.hash_prio[(i+2)%3] += 1;

                // Removing previous entry
                hash_to_member.remove(&entry.hash_val[i].clone());
                hash_to_member.insert(hash.clone(), entry_key);
                entry.hash_val[i] = hash.clone();
                break;
            }
        }

        self.db_main.execute_wparams("UPDATE member SET val_hash1=:vh1, val_prio1=:vp1, val_hash2=:vh2, val_prio2=:vp2, val_hash3=:vh3, val_prio3=:vp3 WHERE id=:id", params!(
            "vh1" => entry.hash_val[0].clone(),
            "vp1" => entry.hash_prio[0],
            "vh2" => entry.hash_val[1].clone(),
            "vp2" => entry.hash_prio[1],
            "vh3" => entry.hash_val[2].clone(),
            "vp3" => entry.hash_prio[2],
            "id" => entry_key
        ));

        Some(hash)
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
        None => content::Json("Error?!".to_string()) // 404?
    }
}

#[derive(Deserialize)]
pub struct PostCreateMember{
    mail: String,
    password: String
}
#[post("/create", data = "<params>")]
pub fn create(me: State<Backend>, params: Json<PostCreateMember>) -> content::Json<String>
{
    content::Json(me.create(&params).to_string())
}

// TODO: Add validation
#[derive(Deserialize)]
pub struct PostDeleteMember{
    id: u32
}
#[delete("/delete", data = "<params>")]
pub fn delete(me: State<Backend>, params: Json<PostDeleteMember>) -> content::Json<String>
{
    content::Json(me.delete(&params).to_string())
}

#[derive(Deserialize)]
pub struct PostLogin{
    mail: String,
    password: String
}
#[post("/login", data = "<params>")]
pub fn login(me: State<Backend>, params: Json<PostLogin>) -> content::Json<String>
{
    match me.login(&params) {
        Some(hash) => content::Json(hash),
        None => content::Json("Error?!".to_string()) // 404 ?
    }
}
