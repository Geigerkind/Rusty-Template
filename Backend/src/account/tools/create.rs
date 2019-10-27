use crate::util::validator;
use crate::util::sha3;
use crate::util::random;
use crate::util::mail;
use crate::util::strformat;
use crate::account::service::create::PostCreateMember;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::material::member::Member;
use crate::account::tools::validator::Validator;
use crate::account::material::account::Account;
use crate::database::tools::mysql::select::Select;
use crate::database::tools::mysql::execute::Execute;
use crate::database::tools::mysql::exists::Exists;
use crate::language::tools::get::Get;
use crate::language::domainvalue::language::Language;
use crate::account::domainvalue::account_information::AccountInformation;
use crate::account::tools::get::GetAccountInformation;

pub trait Create {
  fn create(&self, params: &PostCreateMember) -> Result<AccountInformation, String>;
  fn send_confirmation(&self, params: &ValidationPair, bypass: bool) -> bool;
  fn confirm(&self, id: &str) -> bool;
}

impl Create for Account {
  fn create(&self, params: &PostCreateMember) -> Result<AccountInformation, String>
  {
    if params.nickname.is_empty() {
      return Err("TODO: False".to_string());
    }

    if params.mail.is_empty() {
      return Err("TODO: False".to_string());
    }

    if params.password.is_empty() {
      return Err("TODO: False".to_string());
    }

    if !validator::mail(&params.mail) {
      return Err("TODO: False".to_string());
    }

    // Double spending check
    // We dont validate through the internal data structure because we may have race conditions
    if self.db_main.exists_wparams("SELECT id FROM member WHERE LOWER(mail) = :mail LIMIT 1", params!("mail" => params.mail.clone().to_lowercase()))
    {
      return Err("TODO: False".to_string());
    }

    // Also prevent the same nickname
    if self.db_main.exists_wparams("SELECT id FROM member WHERE LOWER(nickname) = :nickname LIMIT 1", params!("nickname" => params.nickname.clone().to_lowercase()))
    {
      return Err("TODO: False".to_string());
    }

    let salt: String = random::alphanumeric(16);
    let pass: String = sha3::hash(vec![&params.password, &salt]);

    if self.db_main.execute_wparams("INSERT IGNORE INTO member (`mail`, `password`, `nickname`, `joined`) VALUES (:mail, :pass, :nickname, UNIX_TIMESTAMP())", params!(
      "nickname" => params.nickname.clone(),
      "mail" => params.mail.clone(),
      "pass" => pass.clone())
    ) {
      let id: u32;
      { // Keep write locks as short as possible
        let mut member = self.member.write().unwrap();
        id = self.db_main.select_wparams_value("SELECT id FROM member WHERE LOWER(mail) = :mail", &|row|{
          mysql::from_row(row)
        }, params!(
          "mail" => params.mail.to_owned().to_lowercase()
        )).unwrap();
        member.insert(id, Member {
          id,
          nickname: params.nickname.to_owned(),
          mail: params.mail.to_owned(),
          password: pass,
          salt,
          xp: 0,
          mail_confirmed: false,
          forgot_password: false,
          delete_account: false,
          hash_prio: vec![2,2,2],
          hash_val: vec!["none".to_string(), "none".to_string(), "none".to_string()]
        });
      }

      self.send_confirmation(&ValidationPair{hash: String::new(), id}, true);
      return Ok(self.get(id).unwrap());
    }
    return Err("TODO: False".to_string());
  }

  fn send_confirmation(&self, params: &ValidationPair, bypass: bool) -> bool
  {
    if !bypass && !self.validate(params) {
      return false;
    }

    let member = self.member.read().unwrap();
    let entry = member.get(&params.id).unwrap();
    let mail_id = sha3::hash(vec![&params.id.to_string(), &entry.salt]);
    let mail_content = strformat::fmt(self.dictionary.get("create.confirmation.text", Language::English), &vec![&mail_id]);

    if bypass || !entry.mail_confirmed {
      let mut requires_mail_confirmation = self.requires_mail_confirmation.write().unwrap();
      if !requires_mail_confirmation.contains_key(&mail_id) {
        requires_mail_confirmation.insert(mail_id, params.id);
      }
      return mail::send(&entry.mail, &entry.nickname,
        self.dictionary.get("create.confirmation.subject", Language::English), mail_content);
    }
    false
  }

  fn confirm(&self, id: &str) -> bool
  {
    let mut removable = false;
    {
      let requires_mail_confirmation = self.requires_mail_confirmation.read().unwrap();
      match requires_mail_confirmation.get(id) {
        Some(member_id) => {
          if self.db_main.execute_wparams("UPDATE member SET mail_confirmed=1 WHERE id=:id", params!(
            "id" => *member_id
          )) {
            let mut member = self.member.write().unwrap();
            let entry = member.get_mut(member_id).unwrap();
            entry.mail_confirmed = true;
            removable = true;
          }
        },
        None => return false
      }
    }
    if removable {
      let mut  requires_mail_confirmation = self.requires_mail_confirmation.write().unwrap();
      requires_mail_confirmation.remove(id);
      return true;
    }
    false
  }
}