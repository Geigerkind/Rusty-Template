use crate::util::sha3;
use crate::util::mail;
use crate::util::random;
use crate::util::strformat;
use crate::util::validator;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::material::account::Account;
use crate::database::tools::mysql::execute::Execute;
use crate::util::language::tools::get::Get;
use crate::util::language::domainvalue::language::Language;
use crate::account::service::login::PostLogin;
use crate::account::tools::login::Login;
use crate::account::tools::update::Update;

pub trait Forgot {
  fn send_forgot_password(&self, mail: &str) -> Result<(), String>;
  fn recv_forgot_password(&self, id: &str) -> Result<ValidationPair, String>;
}

impl Forgot for Account {
  fn send_forgot_password(&self, mail: &str) -> Result<(), String>
  {
    if !validator::mail(mail) {
      return Err(self.dictionary.get("general.error.invalid.mail", Language::English));
    }

    let forgot_id: String;
    let mut member_id = 0;
    {
      {
        let lower_mail = mail.to_lowercase();
        for member_entry in self.member.read().unwrap().values() {
          if member_entry.mail.to_lowercase() == lower_mail {
            member_id = member_entry.id;
            break;
          }
        }

        let member = self.member.read().unwrap();
        let entry_get = member.get(&member_id);
        if entry_get.is_none() {
          // Don't leak information about existance
          return Ok(());
        }
        let entry = entry_get.unwrap();

        forgot_id = sha3::hash(&[&member_id.to_string(), "forgot", &entry.salt]);
        if !mail::send(&entry.mail, &entry.nickname, self.dictionary.get("forgot.confirmation.subject", Language::English),
          strformat::fmt(self.dictionary.get("forgot.confirmation.text", Language::English), &vec![&forgot_id])){
            return Err(self.dictionary.get("general.error.mail_send", Language::English));
        }
      }
      if self.db_main.execute_wparams("UPDATE member SET forgot_password=1 WHERE id=:id", params!("id" => member_id)) {
        let mut member = self.member.write().unwrap();
        let entry = member.get_mut(&member_id).unwrap();
        entry.forgot_password = true;
      }
    }

    let mut forgot_password = self.forgot_password.write().unwrap();
    forgot_password.insert(forgot_id, member_id);

    Ok(())
  }

  fn recv_forgot_password(&self, id: &str) -> Result<ValidationPair, String>
  {
    let mut removable = false;
    let user_mail;
    let user_pass;
    let user_id;
    {
      let forgot_password = self.forgot_password.read().unwrap();
      match forgot_password.get(id) {
        Some(member_id) => {
          // Sending random generated password
          user_pass = random::alphanumeric(32);
          user_id = *member_id;
          {
            let member = self.member.read().unwrap();
            let entry = member.get(member_id).unwrap();
            user_mail = entry.mail.clone();
            if !mail::send(&entry.mail, &entry.nickname, self.dictionary.get("forgot.information.subject", Language::English),
              strformat::fmt(self.dictionary.get("forgot.information.text", Language::English), &vec![&user_pass])) {
                return Err(self.dictionary.get("general.error.mail_send", Language::English));
            }
          }
          if self.db_main.execute_wparams("UPDATE member SET forgot_password=0 WHERE id=:id", params!(
            "id" => *member_id
          )) {
            let mut member = self.member.write().unwrap();
            let entry = member.get_mut(member_id).unwrap();
            entry.forgot_password = false;
            removable = true;
          }
        },
        None => return Err(self.dictionary.get("forgot.error.no_forgot_issued", Language::English))
      }
    }
    if removable {
      {
        let mut forgot_password = self.forgot_password.write().unwrap();
        forgot_password.remove(id);
      }
      self.update_password(user_id, &user_pass);
      return self.login(&PostLogin {
        mail: user_mail,
        password: user_pass
      });
    }
    Err(self.dictionary.get("general.unknown", Language::English))
  }
}