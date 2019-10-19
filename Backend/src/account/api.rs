use crate::Backend;
use crate::util::Util;

use crate::account::dto::update::PostChangeStr;
use crate::account::dto::create::PostCreateMember;
use crate::account::dto::login::PostLogin;

use crate::account::domainvalue::account_information::AccountInformation;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::material::member::Member;

use std::collections::HashMap;

/**
 * Module implementation
 */
pub trait Account {
    fn init(&self);

    fn get(&self, id: u32) -> Option<AccountInformation>;

    fn create(&self, params: &PostCreateMember) -> bool;
    fn send_confirmation(&self, params: &ValidationPair, bypass: bool) -> bool;
    fn confirm(&self, id: &str) -> bool;

    fn issue_delete(&self, params: &ValidationPair) -> bool;
    fn confirm_delete(&self, id: &str) -> bool;

    fn login(&self, params: &PostLogin) -> Option<String>;
    fn validate(&self, params: &ValidationPair) -> bool;

    fn send_forgot_password(&self, params: &ValidationPair) -> bool;
    fn recv_forgot_password(&self, id: &str) -> bool;

    fn change_name(&self, params: &PostChangeStr) -> bool;
    fn change_password(&self, params: &PostChangeStr) -> Option<String>;
    fn change_mail(&self, params: &PostChangeStr) -> Option<String>;

    fn helper_clear_validation(&self, member_id: &u32, hash_to_member: &mut HashMap<String, u32>, member: &mut HashMap<u32, Member>);
    fn helper_create_validation(&self, member_id: &u32, hash_to_member: &mut HashMap<String, u32>, member: &mut HashMap<u32, Member>) -> String;
}

impl Account for Backend {
    fn init(&self)
    {
        let mut requires_mail_confirmation = self.data_acc.requires_mail_confirmation.write().unwrap();
        let mut forgot_password = self.data_acc.forgot_password.write().unwrap();
        let mut delete_account = self.data_acc.delete_account.write().unwrap();
        let mut hash_to_member = self.data_acc.hash_to_member.write().unwrap();
        let mut member = self.data_acc.member.write().unwrap();

        // We are a little wasteful here because we do not insert it directly but rather create a vector first and then copy it over
        for entry in self.db_main.select("SELECT id, nickname, mail, password, salt, xp, mail_confirmed, forgot_password, delete_account, val_prio1, val_prio2, val_prio3, val_hash1, val_hash2, val_hash3 FROM member", &|mut row|{
            Member {
                id: row.take(0).unwrap(),
                nickname: row.take(1).unwrap(),
                mail: row.take(2).unwrap(),
                password: row.take(3).unwrap(),
                salt: row.take(4).unwrap(),
                xp: row.take(5).unwrap(),
                mail_confirmed: row.take(6).unwrap(),
                forgot_password: row.take(7).unwrap(),
                delete_account: row.take(8).unwrap(),
                hash_prio: vec![row.take(9).unwrap(), row.take(10).unwrap(), row.take(11).unwrap()],
                hash_val: vec![row.take(12).unwrap(), row.take(13).unwrap(), row.take(14).unwrap()]
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
                forgot_password.insert(Util::sha3(self, vec![&entry.id.to_string(), "forgot", &entry.salt]), entry.id);
            }

            // Init remaining delete mails
            if entry.delete_account {
                delete_account.insert(Util::sha3(self, vec![&entry.id.to_string(), "delete", &entry.salt]), entry.id);
            }

            member.insert(entry.id, entry);
        }
    }

    fn create(&self, params: &PostCreateMember) -> bool
    {
        if params.mail.is_empty() {
            return false;
        }

        if params.password.is_empty() {
            return false;
        }

        if !Util::is_valid_mail(self, &params.mail) {
            return false;
        }

        // Double spending check
        // We dont validate through the internal data structure because we may have race conditions
        if self.db_main.exists_wparams("SELECT id FROM member WHERE LOWER(mail) = :mail LIMIT 1", params!("mail" => params.mail.clone().to_lowercase())) 
        {
            return false;
        }

        // Also prevent the same nickname
        if self.db_main.exists_wparams("SELECT id FROM member WHERE LOWER(nickname) = :nickname LIMIT 1", params!("nickname" => params.nickname.clone().to_lowercase())) 
        {
            return false;
        }

        let salt: String = Util::random_str(self, 16);
        let pass: String = Util::sha3(self, vec![&params.password, &salt]);

        if self.db_main.execute_wparams("INSERT IGNORE INTO member (`mail`, `password`, `nickname`, `joined`) VALUES (:mail, :pass, :nickname, UNIX_TIMESTAMP())", params!(
            "nickname" => params.nickname.clone(),
            "mail" => params.mail.clone(),
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
                    nickname: params.nickname.to_owned(),
                    mail: params.mail.to_owned(),
                    password: pass,
                    salt: salt.clone(), 
                    xp: 0,
                    mail_confirmed: false,
                    forgot_password: false,
                    delete_account: false,
                    hash_prio: vec![2,2,2],
                    hash_val: vec!["none".to_string(), "none".to_string(), "none".to_string()]
                });
            }

            self.send_confirmation(&ValidationPair{hash: String::new(), id:id}, true);
        }
        true
    }

    fn issue_delete(&self, params: &ValidationPair) -> bool
    {
        if !self.validate(params) {
            return false; // Rather return errors?
        }

        let delete_id: String;
        {
            {
                let member = self.data_acc.member.read().unwrap();
                let entry = member.get(&params.id).unwrap();
                delete_id = Util::sha3(self, vec![&params.id.to_string(), "delete", &entry.salt]);
                if !Util::send_mail(self, &entry.mail, "TODO: Username", "Delete account utility", &vec!["TODO: FANCY TEXT\nhttps://jaylapp.dev/API/account/delete/confirm/", &delete_id].concat()){
                    return false;
                }
            }
            if !self.db_main.execute_wparams("UPDATE member SET delete_account=1 WHERE id=:id", params!("id" => params.id)) {
                return false;
            } else {
                let mut member = self.data_acc.member.write().unwrap();
                let entry = member.get_mut(&params.id).unwrap();
                entry.delete_account = true;
            }
        }

        let mut delete_account = self.data_acc.delete_account.write().unwrap();
        delete_account.insert(delete_id, params.id);

        true
    }

    fn confirm_delete(&self, id: &str) -> bool
    {
        let mut removable = false;
        {
            let delete_account = self.data_acc.delete_account.read().unwrap();
            match delete_account.get(id) {
                Some(member_id) => {
                    if self.db_main.execute_wparams("DELETE FROM member WHERE id = :id", params!(
                        "id" => *member_id
                    )) {
                        let mut hash_to_member = self.data_acc.hash_to_member.write().unwrap();
                        let mut member = self.data_acc.member.write().unwrap();
                        self.helper_clear_validation(member_id, &mut (*hash_to_member), &mut (*member));
                        member.remove(member_id);
                        removable = true;
                    }
                },
                None => return false
            }
        }
        if removable {
            let mut delete_account = self.data_acc.delete_account.write().unwrap();
            delete_account.remove(id);
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

        Some(self.helper_create_validation(&entry_key, &mut(*hash_to_member), &mut(*member)))
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

        let forgot_id: String;
        {
            {
                let member = self.data_acc.member.read().unwrap();
                let entry = member.get(&params.id).unwrap();
                forgot_id = Util::sha3(self, vec![&params.id.to_string(), "forgot", &entry.salt]);
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

    fn change_name(&self, params: &PostChangeStr) -> bool
    {
        if !self.validate(&params.validation) {
            return false; // Rather return errors?
        }

        if params.content.is_empty() {
            return false;
        }

        // Check if the name exists already
        let lower_name = params.content.to_lowercase();
        for (_, entry) in &(*self.data_acc.member.read().unwrap()) {
            if entry.nickname.to_lowercase() == lower_name
                && entry.id != params.validation.id 
            {
                return false;
            }
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

    fn change_password(&self, params: &PostChangeStr) -> Option<String>
    {
        if !self.validate(&params.validation) {
            return None; // Rather return errors?
        }

        if params.content.is_empty() {
            return None;
        }

        let hash: String;
        {
            let member = self.data_acc.member.read().unwrap();
            let entry = member.get(&params.validation.id).unwrap();
            hash = Util::sha3(self, vec![&entry.mail, &params.content, &entry.salt]);
        }
        
        if self.db_main.execute_wparams("UPDATE member SET password=:password WHERE id=:id", params!(
            "password" => hash.clone(),
            "id" => params.validation.id
        )) {
            let mut hash_to_member = self.data_acc.hash_to_member.write().unwrap();
            let mut member = self.data_acc.member.write().unwrap();
            self.helper_clear_validation(&params.validation.id, &mut(*hash_to_member), &mut(*member));
            {
                let entry = member.get_mut(&params.validation.id).unwrap();
                entry.password = hash.to_owned();
            }
            return Some(self.helper_create_validation(&params.validation.id, &mut(*hash_to_member), &mut(*member)));
        }

        None
    }

    fn change_mail(&self, params: &PostChangeStr) -> Option<String>
    {
        if !self.validate(&params.validation) {
            return None; // Rather return errors?
        }

        if !Util::is_valid_mail(self, &params.content) {
            return None;
        }

        // Check if the mail exists already
        let lower_mail = params.content.to_lowercase();
        for (_, entry) in &(*self.data_acc.member.read().unwrap()) {
            if entry.mail.to_lowercase() == lower_mail
                && entry.id != params.validation.id 
            {
                return None;
            }
        }

        if self.db_main.execute_wparams("UPDATE member SET mail=:mail WHERE id=:id", params!(
            "mail" => params.content.clone(),
            "id" => params.validation.id
        )) {
            let mut hash_to_member = self.data_acc.hash_to_member.write().unwrap();
            let mut member = self.data_acc.member.write().unwrap();
            self.helper_clear_validation(&params.validation.id, &mut(*hash_to_member), &mut(*member));
            {
                let entry = member.get_mut(&params.validation.id).unwrap();
                entry.mail = params.content.to_owned();
            }
            return Some(self.helper_create_validation(&params.validation.id, &mut(*hash_to_member), &mut(*member)));
        }

        None
    }

    fn send_confirmation(&self, params: &ValidationPair, bypass: bool) -> bool
    {
        if !bypass && !self.validate(params) {
            return false;
        }

        let member = self.data_acc.member.read().unwrap();
        let entry = member.get(&params.id).unwrap();
        let mail_id = Util::sha3(self, vec![&params.id.to_string(), &entry.salt]);
        let subject = "TODO: Confirm your mail!";
        let text = &vec!["TODO: Heartwarming welcome text\nhttps://jaylapp.dev/API/account/confirm/", &mail_id].concat();

        if bypass || !entry.mail_confirmed {
            let mut requires_mail_confirmation = self.data_acc.requires_mail_confirmation.write().unwrap();
            if !requires_mail_confirmation.contains_key(&mail_id) {
                requires_mail_confirmation.insert(mail_id, params.id);
            }
            return Util::send_mail(self, &entry.mail, &entry.nickname, subject, text);
        } 
        false
    }

    // Helper functions
    fn helper_clear_validation(&self, member_id: &u32, hash_to_member: &mut HashMap<String, u32>, member: &mut HashMap<u32, Member>)
    {
        let entry = member.get_mut(member_id).unwrap();
        for i in 0..2 {
            if entry.hash_val[i] != "none" {
                hash_to_member.remove(&entry.hash_val[i]);
            }
            entry.hash_val[i] = "none".to_string();
            entry.hash_prio[i] = 2;
        }
    }

    fn helper_create_validation(&self, member_id: &u32, hash_to_member: &mut HashMap<String, u32>, member: &mut HashMap<u32, Member>) -> String
    {
        let entry = member.get_mut(member_id).unwrap();
            
        // Generate a 128 bit salt for our validation hash
        let salt: String = Util::random_str(self, 16);
        let hash: String = Util::sha3(self, vec![&entry.mail, &entry.password, &salt]);

        // Replace by using the Least recently used strategy
        for i in 0..2 {
            if entry.hash_prio[i] >= 2 {
                // Adjusting prios
                entry.hash_prio[i] = 0;
                entry.hash_prio[(i+1)%3] += 1;
                entry.hash_prio[(i+2)%3] += 1;

                // Removing previous entry
                hash_to_member.remove(&entry.hash_val[i].clone());
                hash_to_member.insert(hash.clone(), *member_id);
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
            "id" => *member_id
        ));

        hash
    }
    
}