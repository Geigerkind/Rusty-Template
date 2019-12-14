use language::domain_value::Language;
use language::tools::Get;
use mail;
use mysql_connection::tools::{ Execute, Select };
use str_util::{random, sha3, strformat};
use validator::tools::{valid_password, valid_nickname, valid_mail};
use validator::domain_value::PasswordFailure;

use crate::modules::account::material::{Account, Member, APIToken};
use crate::modules::account::tools::Token;
use crate::modules::account::dto::Failure;

pub trait Create {
  fn create(&self, mail: &str, nickname: &str, password: &str) -> Result<APIToken, Failure>;
  fn send_confirmation(&self, member_id: u32) -> bool;
  fn confirm(&self, id: &str) -> bool;
}

impl Create for Account {
  fn create(&self, mail: &str, nickname: &str, password: &str) -> Result<APIToken, Failure>
  {
    if !valid_mail(mail) {
      return Err(Failure::InvalidMail);
    }

    if !valid_nickname(nickname) {
      return Err(Failure::InvalidNickname);
    }

    match valid_password(password) {
      Err(PasswordFailure::TooFewCharacters) => return Err(Failure::PasswordTooShort),
      Err(PasswordFailure::Pwned(num_pwned)) => return Err(Failure::PwnedPassword(num_pwned)),
      Ok(_) => ()
    };

    // The following part needs to be transactional
    let member_id: u32;
    {
      let mut member = self.member.write().unwrap();
      let lower_mail = mail.to_lowercase();
      let lower_nickname = nickname.to_lowercase();
      for entry in member.values() {
        if entry.mail == lower_mail || entry.new_mail == lower_mail {
          return Err(Failure::MailIsInUse);
        } else if entry.nickname.to_lowercase() == lower_nickname {
          return Err(Failure::NicknameIsInUse);
        }
      }

      let salt: String = random::alphanumeric(16);
      let pass: String = sha3::hash(&[password, &salt]);

      if self.db_main.execute_wparams("INSERT IGNORE INTO member (`mail`, `password`, `nickname`, `salt`, `joined`) VALUES (:mail, :pass, :nickname, :salt, UNIX_TIMESTAMP())", params!(
      "nickname" => nickname.clone(),
      "mail" => lower_mail.clone(),
      "pass" => pass.clone(),
      "salt" => salt.clone()
      ),
      ) {
        member_id = self.db_main.select_wparams_value("SELECT id FROM member WHERE mail = :mail", &|mut row| {
          row.take(0).unwrap()
        }, params!(
        "mail" => lower_mail.clone()
      )).unwrap();
        member.insert(member_id, Member {
          id: member_id,
          nickname: nickname.to_owned(),
          mail: lower_mail.clone(),
          password: pass,
          salt,
          mail_confirmed: false,
          forgot_password: false,
          delete_account: false,
          new_mail: String::new()
        });
      } else {
        return Err(Failure::Unknown);
      }
    }

    self.send_confirmation(member_id);
    return self.create_token(
      &self.dictionary.get("general.login", Language::English),
      member_id, time_util::get_ts_from_now_in_secs(7));
  }

  fn send_confirmation(&self, member_id: u32) -> bool
  {
    // Sub-optimal code but this follows the convention to always lock in the same order
    let mut requires_mail_confirmation = self.requires_mail_confirmation.write().unwrap();

    let member = self.member.read().unwrap();
    let entry = member.get(&member_id).unwrap();
    let mail_id = sha3::hash(&[&member_id.to_string(), "mail", &entry.salt]);
    let mail_content = strformat::fmt(self.dictionary.get("create.confirmation.text", Language::English), &[&mail_id]);

    if !entry.mail_confirmed {
      if !requires_mail_confirmation.contains_key(&mail_id) {
        requires_mail_confirmation.insert(mail_id, member_id);
      }
      return mail::send(&entry.mail, &entry.nickname,
                        self.dictionary.get("create.confirmation.subject", Language::English), mail_content);
    }
    false
  }

  fn confirm(&self, id: &str) -> bool
  {
    let mut requires_mail_confirmation = self.requires_mail_confirmation.write().unwrap();
    let mut member = self.member.write().unwrap();
    let confirm_id_res = requires_mail_confirmation.get(id);

    if confirm_id_res.is_none() {
      return false;
    }

    let member_id = *confirm_id_res.unwrap();
    if self.db_main.execute_wparams("UPDATE member SET mail_confirmed=1 WHERE id=:id", params!(
      "id" => member_id
    )) {
      let entry = member.get_mut(&member_id).unwrap();
      entry.mail_confirmed = true;
      requires_mail_confirmation.remove(id);
      return true;
    }
    return false;
  }
}