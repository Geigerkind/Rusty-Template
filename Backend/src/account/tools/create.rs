use language::domainvalue::language::Language;
use language::get::Get;
use mail;
use str_util::{random, sha3, strformat};
use validator;
use validator::domainvalue::password_failure::PasswordFailure;

use crate::account::domainvalue::post_create_member::PostCreateMember;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::material::account::Account;
use crate::account::material::member::Member;
use crate::account::tools::token::Token;
use crate::database::tools::mysql::execute::Execute;
use crate::database::tools::mysql::exists::Exists;
use crate::database::tools::mysql::select::Select;

pub trait Create {
  fn create(&self, params: &PostCreateMember) -> Result<ValidationPair, String>;
  fn send_confirmation(&self, params: &ValidationPair) -> bool;
  fn confirm(&self, id: &str) -> bool;
}

impl Create for Account {
  fn create(&self, params: &PostCreateMember) -> Result<ValidationPair, String>
  {
    if !validator::mail(&params.mail) {
      return Err(self.dictionary.get("general.error.invalid.mail", Language::English));
    }

    if !validator::nickname(&params.nickname) {
      return Err(self.dictionary.get("general.error.invalid.nickname", Language::English));
    }

    match validator::password(&params.password) {
      Err(PasswordFailure::TooFewCharacters) => return Err(self.dictionary.get("general.error.password.length", Language::English)),
      Err(PasswordFailure::Pwned(num_pwned)) => return Err(strformat::fmt(self.dictionary.get("general.error.password.pwned", Language::English), &[&num_pwned.to_string()])),
      Ok(_) => ()
    };

    // Double spending check
    // We dont validate through the internal data structure because we may have race conditions
    let lower_mail = params.mail.clone().to_lowercase();
    if self.db_main.exists_wparams("SELECT id FROM member WHERE mail = :mail LIMIT 1", params!("mail" => lower_mail.clone()))
    {
      return Err(self.dictionary.get("create.error.taken.mail", Language::English));
    }

    // Also prevent the same nickname
    if self.db_main.exists_wparams("SELECT id FROM member WHERE LOWER(nickname) = :nickname LIMIT 1", params!("nickname" => params.nickname.clone().to_lowercase()))
    {
      return Err(self.dictionary.get("create.error.taken.nickname", Language::English));
    }

    let salt: String = random::alphanumeric(16);
    let pass: String = sha3::hash(&[&params.password, &salt]);

    if self.db_main.execute_wparams("INSERT IGNORE INTO member (`mail`, `password`, `nickname`, `joined`) VALUES (:mail, :pass, :nickname, UNIX_TIMESTAMP())", params!(
      "nickname" => params.nickname.clone(),
      "mail" => params.mail.clone(),
      "pass" => pass.clone()),
    ) {
      let id: u32;
      { // Keep write locks as short as possible
        let mut member = self.member.write().unwrap();
        id = self.db_main.select_wparams_value("SELECT id FROM member WHERE mail = :mail", &|row| {
          mysql::from_row(row)
        }, params!(
          "mail" => lower_mail.clone()
        )).unwrap();
        member.insert(id, Member {
          id,
          nickname: params.nickname.to_owned(),
          mail: lower_mail.clone(),
          password: pass,
          salt,
          mail_confirmed: false,
          forgot_password: false,
          delete_account: false,
        });
      }

      return match self.create_validation_unsafe(
        &self.dictionary.get("general.login", Language::English),
        id, time_util::get_ts_from_now_in_secs(30)) {
        Ok(val_pair) => {
          self.send_confirmation(&val_pair);
          Ok(val_pair)
        }
        Err(err_str) => Err(err_str)
      };
    }
    return Err(self.dictionary.get("general.error.unknown", Language::English));
  }

  fn send_confirmation(&self, params: &ValidationPair) -> bool
  {
    if !self.validate(params) {
      return false;
    }

    let member = self.member.read().unwrap();
    let entry = member.get(&params.member_id).unwrap();
    let mail_id = sha3::hash(&[&params.member_id.to_string(), &entry.salt]);
    let mail_content = strformat::fmt(self.dictionary.get("create.confirmation.text", Language::English), &[&mail_id]);

    if !entry.mail_confirmed {
      let mut requires_mail_confirmation = self.requires_mail_confirmation.write().unwrap();
      if !requires_mail_confirmation.contains_key(&mail_id) {
        requires_mail_confirmation.insert(mail_id, params.member_id);
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
        }
        None => return false
      }
    }
    if removable {
      let mut requires_mail_confirmation = self.requires_mail_confirmation.write().unwrap();
      requires_mail_confirmation.remove(id);
      return true;
    }
    false
  }
}