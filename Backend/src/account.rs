use crate::Backend;
use crate::util::Util;

use rocket::response::content;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json::to_string;

pub struct Member {
    id: u32,
    nickname: String,
    mail: String,
    password: String,
    salt: String,
    xp: u32,
    mail_confirmed: bool,
    forgot_password: bool,
    hash_prio: Vec<u8>,
    hash_val: Vec<String>
}

#[derive(Serialize)]
pub struct AccountInformation {
    mail: String,
    xp: u32
}

#[derive(Deserialize)]
pub struct ValidationPair {
    hash: String,
    id: u32
}

/**
 * Dominating data structure supporting this module
 */
use std::sync::RwLock;
use std::collections::HashMap;
pub struct AccountData {
    member: RwLock<HashMap<u32, Member>>,
    hash_to_member: RwLock<HashMap<String, u32>>,
    requires_mail_confirmation: RwLock<HashMap<String, u32>>,
    forgot_password: RwLock<HashMap<String, u32>>,
}
impl AccountData {
    pub fn new() -> Self
    {
        AccountData {
            member: RwLock::new(HashMap::new()),
            hash_to_member: RwLock::new(HashMap::new()),
            requires_mail_confirmation: RwLock::new(HashMap::new()),
            forgot_password: RwLock::new(HashMap::new()),
        }
    }
}

/**
 * Module implementation
 */
pub trait Account {
    fn init(&self);

    fn get(&self, id: u32) -> Option<AccountInformation>;

    fn create(&self, params: &PostCreateMember) -> bool;
    fn confirm(&self, id: &str) -> bool;
    fn delete(&self, params: &ValidationPair) -> bool;
    
    fn login(&self, params: &PostLogin) -> Option<String>;
    fn validate(&self, params: &ValidationPair) -> bool;

    fn send_forgot_password(&self, params: &ValidationPair) -> bool;
    fn recv_forgot_password(&self, id: &str) -> bool;

    fn change_name(&self, params: &PostChangeStr) -> bool;
    fn change_password(&self, params: &PostChangeStr) -> bool;
    fn change_mail(&self, params: &PostChangeStr) -> bool;
}

impl Account for Backend {
    fn init(&self)
    {
        let mut requires_mail_confirmation = self.data_acc.requires_mail_confirmation.write().unwrap();
        let mut forgot_password = self.data_acc.forgot_password.write().unwrap();
        let mut hash_to_member = self.data_acc.hash_to_member.write().unwrap();
        let mut member = self.data_acc.member.write().unwrap();

        // We are a little wasteful here because we do not insert it directly but rather create a vector first and then copy it over
        for entry in self.db_main.select("SELECT id, mail, password, salt, mail_confirmed, forgot_password, val_hash1, val_prio1, val_hash2, val_prio2, val_hash3, val_prio3 FROM member", &|row|{
            let (id, mail, pass, salt, mail_confirmed, forgot_password, val_hash1, val_prio1, val_hash2, val_prio2, val_hash3, val_prio3) = mysql::from_row(row);
            Member {
                id: id,
                nickname: String::new(), //TODO
                mail: mail,
                password: pass,
                salt: salt,
                xp: 0,
                mail_confirmed: mail_confirmed,
                forgot_password: forgot_password,
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

            // Init remaining confirmation mails
            if !entry.mail_confirmed {
                requires_mail_confirmation.insert(Util::sha3(self, vec![&entry.id.to_string(), &entry.salt]), entry.id);
            }

            // Init remaining forgot password mails
            if entry.forgot_password {
                forgot_password.insert(Util::sha3(self, vec![&entry.id.to_string(), "forgot"]), entry.id);
            }

            member.insert(entry.id, entry);
        }
    }

    // TODO: Check for validity of inputs!
    fn create(&self, params: &PostCreateMember) -> bool
    {
        // Double spending check
        // We dont validate throguh the internal data structure because we may have race conditions
        if self.db_main.exists_wparams("SELECT id FROM member WHERE LOWER(mail) = :mail LIMIT 1", params!("mail" => params.mail.to_owned().to_lowercase())) 
        {
            return false;
        }

        let salt: String = Util::random_str(self, 16);
        let pass: String = Util::sha3(self, vec![&params.password, &salt]);

        if self.db_main.execute_wparams("INSERT IGNORE INTO member (`mail`, `password`, `joined`) VALUES (:mail, :pass, UNIX_TIMESTAMP())", params!(
            "mail" => params.mail.to_owned(),
            "pass" => pass.clone())
        ) {
            let id: u32;
            { // Keep write locks as short as possible
                let mut member = self.data_acc.member.write().unwrap();
                id = self.db_main.select_wparams_value("SELECT id FROM member WHERE LOWER(mail) = :mail", &|row|{
                    let res = mysql::from_row(row);
                    res
                }, params!(
                    "mail" => params.mail.to_owned().to_lowercase()
                )).unwrap();
                member.insert(id, Member {
                    id: id,
                    nickname: String::new(), // TODO
                    mail: params.mail.to_owned(),
                    password: pass,
                    salt: salt.clone(), 
                    xp: 0,
                    mail_confirmed: false,
                    forgot_password: false,
                    hash_prio: vec![2,2,2],
                    hash_val: vec!["none".to_string(), "none".to_string(), "none".to_string()]
                });
            }

            // Sending a confirmation mail
            let mail_id = Util::sha3(self, vec![&id.to_string(), &salt]);
            Util::send_mail(self, &params.mail, "TODO: Nickname", "TODO: Confirm your mail!", &vec!["TODO: Heartwarming welcome text\nhttps://jaylapp.dev/API/account/confirm/", &mail_id].concat());

            let mut requires_mail_confirmation = self.data_acc.requires_mail_confirmation.write().unwrap();
            requires_mail_confirmation.insert(mail_id, id);
        }
        true
    }

    // We might consider to send a mail first!
    fn delete(&self, params: &ValidationPair) -> bool
    {
        // This also makes sure that the user actually exists
        if !self.validate(params) {
            return false; // Rather return errors?
        }

        if self.db_main.execute_wparams("DELETE FROM member WHERE id = :id", params!(
            "id" => params.id
        )) {
            let mut hash_to_member = self.data_acc.hash_to_member.write().unwrap();
            let mut member = self.data_acc.member.write().unwrap();
            // Creating this scope to reduce the lifetime of the borrow
            {
                let entry = member.get(&params.id).unwrap();
                for i in 0..2 {
                    if entry.hash_val[i] != "none" {
                        hash_to_member.remove(&entry.hash_val[i]);
                    }
                }
            }
            member.remove(&params.id);
            return true;
        }
        false
    }

    fn get(&self, id: u32) -> Option<AccountInformation>
    {
        let member = self.data_acc.member.read().unwrap();
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
        let mut hash_to_member = self.data_acc.hash_to_member.write().unwrap();
        let mut member = self.data_acc.member.write().unwrap();

        let lower_mail = params.mail.to_lowercase();
        let mut entry_key: u32 = 0;
        for (id, entry) in &(*member) {
            if entry.mail.to_lowercase() != lower_mail { continue; }
            if entry.password != Util::sha3(self, vec![&params.password, &entry.salt]) { break; } // Password is wrong
            entry_key = *id;
            break
        }
        if entry_key == 0 { return None; }
        let entry = member.get_mut(&entry_key).unwrap();
            
        // Generate a 128 bit salt for our validation hash
        let salt: String = Util::random_str(self, 16);
        let hash: String = Util::sha3(self, vec![&entry.mail, &params.password, &salt]);

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

    fn validate(&self, params: &ValidationPair) -> bool
    {
        let hash_to_member = self.data_acc.hash_to_member.read().unwrap();
        match hash_to_member.get(&params.hash) {
            Some(id) => {
                // Doing it this way, because write locks need to be avoided
                let mut work_key = 3;
                {
                    // Updating the prios if necessary
                    let member = self.data_acc.member.read().unwrap();
                    let entry = member.get(id).unwrap();
                    // We need to find the index first
                    for i in 0..2 {
                        if entry.hash_val[i] == params.hash {
                            if entry.hash_prio[i] != 0 {
                                work_key = i;
                            }
                            break;
                        }
                    }
                }
                
                if work_key < 3 {
                    let mut member = self.data_acc.member.write().unwrap();
                    let entry = member.get_mut(id).unwrap();

                    // Adjusting prios
                    entry.hash_prio[work_key] = 0;
                    entry.hash_prio[(work_key+1)%3] += 1;
                    entry.hash_prio[(work_key+2)%3] += 1;

                    self.db_main.execute_wparams("UPDATE member SET val_prio1=:vp1, val_prio2=:vp2, val_prio3=:vp3 WHERE id=:id", params!(
                        "vp1" => entry.hash_prio[0],
                        "vp2" => entry.hash_prio[1],
                        "vp3" => entry.hash_prio[2],
                        "id" => *id
                    ));
                }

                *id == params.id
            },
            None => false
        }
    }

    fn confirm(&self, id: &str) -> bool
    {
        let mut removable = false;
        {
            let requires_mail_confirmation = self.data_acc.requires_mail_confirmation.read().unwrap();
            match requires_mail_confirmation.get(id) {
                Some(member_id) => {
                    if self.db_main.execute_wparams("UPDATE member SET mail_confirmed=1 WHERE id=:id", params!(
                        "id" => *member_id
                    )) {
                        let mut member = self.data_acc.member.write().unwrap();
                        let entry = member.get_mut(member_id).unwrap();
                        entry.mail_confirmed = true;
                        removable = true;
                    }
                },
                None => return false
            }
        }
        if removable {
            let mut  requires_mail_confirmation = self.data_acc.requires_mail_confirmation.write().unwrap();
            requires_mail_confirmation.remove(id);
            return true;
        }
        false
    }

    fn send_forgot_password(&self, params: &ValidationPair) -> bool
    {
        if !self.validate(params) {
            return false; // Rather return errors?
        }

        let forgot_id = Util::sha3(self, vec![&params.id.to_string(), "forgot"]);
        {
            {
                let member = self.data_acc.member.read().unwrap();
                let entry = member.get(&params.id).unwrap();
                if !Util::send_mail(self, &entry.mail, "TODO: Username", "Forgot password utility", &vec!["TODO: FANCY TEXT\nhttps://jaylapp.dev/API/account/forgot/confirm/", &forgot_id].concat()){
                    return false;
                }
            }
            if !self.db_main.execute_wparams("UPDATE member SET forgot_password=1 WHERE id=:id", params!("id" => params.id)) {
                return false;
            } else {
                let mut member = self.data_acc.member.write().unwrap();
                let entry = member.get_mut(&params.id).unwrap();
                entry.forgot_password = true;
            }
        }

        let mut forgot_password = self.data_acc.forgot_password.write().unwrap();
        forgot_password.insert(forgot_id, params.id);

        true
    }

    fn recv_forgot_password(&self, id: &str) -> bool
    {
        let mut removable = false;
        {
            let forgot_password = self.data_acc.forgot_password.read().unwrap();
            match forgot_password.get(id) {
                Some(member_id) => {
                    // Sending random generated password
                    let new_pass = Util::random_str(self, 16);
                    {
                        let member = self.data_acc.member.read().unwrap();
                        let entry = member.get(member_id).unwrap();
                        if Util::send_mail(self, &entry.mail, "TODO: username", "TODO: Forgot password utility", &vec!["TODO: Text\n New Password: ", &new_pass].concat()) {
                            return false;
                        }
                    }
                    if self.db_main.execute_wparams("UPDATE member SET forgot_password=0, password=:pass WHERE id=:id", params!(
                        "pass" => new_pass,
                        "id" => *member_id
                    )) {
                        let mut member = self.data_acc.member.write().unwrap();
                        let entry = member.get_mut(member_id).unwrap();
                        entry.forgot_password = false;
                        removable = true;
                    }
                },
                None => return false
            }
        }
        if removable {
            let mut  forgot_password = self.data_acc.forgot_password.write().unwrap();
            forgot_password.remove(id);
            return true;
        }
        false
    }

    // TODO: Validate name
    fn change_name(&self, params: &PostChangeStr) -> bool
    {
        if !self.validate(&params.validation) {
            return false; // Rather return errors?
        }

        if self.db_main.execute_wparams("UPDATE member SET nickname=:nickname WHERE id=:id", params!(
            "nickname" => params.content.clone(),
            "id" => params.validation.id
        )) {
            let mut member = self.data_acc.member.write().unwrap();
            let entry = member.get_mut(&params.validation.id).unwrap();
            entry.nickname = params.content.to_owned();
            return true;
        }

        false
    }

    fn change_password(&self, params: &PostChangeStr) -> bool
    {
        if !self.validate(&params.validation) {
            return false; // Rather return errors?
        }

        if self.db_main.execute_wparams("UPDATE member SET password=:password WHERE id=:id", params!(
            "password" => params.content.clone(),
            "id" => params.validation.id
        )) {
            let mut member = self.data_acc.member.write().unwrap();
            let entry = member.get_mut(&params.validation.id).unwrap();
            entry.password = params.content.to_owned();
            return true;
        }

        false
    }

    // TODO: Maybe send confirmation mail?
    fn change_mail(&self, params: &PostChangeStr) -> bool
    {
        if !self.validate(&params.validation) {
            return false; // Rather return errors?
        }

        if self.db_main.execute_wparams("UPDATE member SET mail=:mail WHERE id=:id", params!(
            "mail" => params.content.clone(),
            "id" => params.validation.id
        )) {
            let mut member = self.data_acc.member.write().unwrap();
            let entry = member.get_mut(&params.validation.id).unwrap();
            entry.mail = params.content.to_owned();
            return true;
        }

        false
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

#[get("/confirm/<id>")]
pub fn confirm(me: State<Backend>, id: String) -> content::Json<String>
{
    content::Json(me.confirm(&id).to_string())
}

#[get("/forgot/confirm/<id>")]
pub fn rcv_forgot(me: State<Backend>, id: String) -> content::Json<String>
{
    content::Json(me.recv_forgot_password(&id).to_string())
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

#[post("/forgot/send", data = "<params>")]
pub fn send_forgot(me: State<Backend>, params: Json<ValidationPair>) -> content::Json<String>
{
    content::Json(me.send_forgot_password(&params).to_string())
}

#[delete("/delete", data = "<params>")]
pub fn delete(me: State<Backend>, params: Json<ValidationPair>) -> content::Json<String>
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

#[derive(Deserialize)]
pub struct PostChangeStr {
    content: String,
    validation: ValidationPair
}
#[post("/update/password", data = "<params>")]
pub fn update_pass(me: State<Backend>, params: Json<PostChangeStr>) -> content::Json<String>
{
    content::Json(me.change_password(&params).to_string())
}
#[post("/update/nickname", data = "<params>")]
pub fn update_nickname(me: State<Backend>, params: Json<PostChangeStr>) -> content::Json<String>
{
    content::Json(me.change_name(&params).to_string())
}
#[post("/update/mail", data = "<params>")]
pub fn update_mail(me: State<Backend>, params: Json<PostChangeStr>) -> content::Json<String>
{
    content::Json(me.change_mail(&params).to_string())
}