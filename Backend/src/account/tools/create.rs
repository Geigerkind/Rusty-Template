use crate::util::sha3;
use crate::util::random;
use crate::util::mail;
use crate::util::strformat;
use crate::util::validator::tools::valid;
use crate::util::validator::domainvalue::password_failure::PasswordFailure;
use crate::util::language::tools::get::Get;
use crate::util::language::domainvalue::language::Language;
use crate::account::service::create::PostCreateMember;
use crate::account::domainvalue::validation_pair::ValidationPair;
use crate::account::material::member::Member;
use crate::account::tools::validator::Validator;
use crate::account::material::account::Account;
use crate::database::tools::mysql::select::Select;
use crate::database::tools::mysql::execute::Execute;
use crate::database::tools::mysql::exists::Exists;

pub trait Create {
  fn create(&self, params: &PostCreateMember) -> Result<ValidationPair, String>;
  fn send_confirmation(&self, params: &ValidationPair) -> bool;
  fn confirm(&self, id: &str) -> bool;
}

impl Create for Account {
  fn create(&self, params: &PostCreateMember) -> Result<ValidationPair, String>
  {
    if !valid::mail(&params.mail) {
      return Err(self.dictionary.get("general.error.invalid.mail", Language::English));
    }

    if !valid::nickname(&params.nickname) {
      return Err(self.dictionary.get("general.error.invalid.nickname", Language::English));
    }

    match valid::password(&params.password) {
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
      "pass" => pass.clone())
    ) {
      let id: u32;
      { // Keep write locks as short as possible
        let mut member = self.member.write().unwrap();
        id = self.db_main.select_wparams_value("SELECT id FROM member WHERE mail = :mail", &|row|{
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
          xp: 0,
          mail_confirmed: false,
          forgot_password: false,
          delete_account: false,
          hash_prio: vec![2,2,2],
          hash_val: vec!["none".to_string(), "none".to_string(), "none".to_string()]
        });
      }

      let val_pair = self.helper_create_validation(id);
      self.send_confirmation(&val_pair);
      return Ok(val_pair);
    }
    return Err(self.dictionary.get("general.error.unknown", Language::English));
  }

  fn send_confirmation(&self, params: &ValidationPair) -> bool
  {
    if !self.validate(params) {
      return false;
    }

    let member = self.member.read().unwrap();
    let entry = member.get(&params.id).unwrap();
    let mail_id = sha3::hash(&[&params.id.to_string(), &entry.salt]);
    let mail_content = strformat::fmt(self.dictionary.get("create.confirmation.text", Language::English), &[&mail_id]);

    if !entry.mail_confirmed {
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