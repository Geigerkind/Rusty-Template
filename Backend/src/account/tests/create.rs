#[cfg(test)]
mod tests {
  use crate::account::material::account::Account;
  use crate::account::tools::create::Create;
  use crate::account::service::create::PostCreateMember;
  use crate::database::tools::mysql::execute::Execute;
  use crate::account::tools::get::GetAccountInformation;
  use crate::util::sha3;

  #[test]
  fn create_account() {
    let account = Account::default();
    let acc_mail = "mail@jaylapp.dev";
    let post_obj = PostCreateMember {
      nickname: "NickName".to_string(),
      mail: acc_mail.to_string(),
      password: "Password123456".to_string()
    };

    let account_information = account.create(&post_obj).unwrap();
    assert_eq!(account_information.mail, acc_mail);

    account.db_main.execute("DELETE FROM member WHERE mail='mail@jaylapp.dev'");
  }

  #[test]
  fn mail_twice() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "BlaNameqqweq".to_string(),
      mail: "bla@jaylapp.dev".to_string(),
      password: "Password123456".to_string()
    };

    let _ = account.create(&post_obj);
    let is_err: bool = match account.create(&post_obj) {
      Ok(_) => false,
      Err(_) => true
    };
    assert!(is_err);

    account.db_main.execute("DELETE FROM member WHERE mail='bla@jaylapp.dev'");
  }

  #[test]
  fn nickname_twice() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "BlaName".to_string(),
      mail: "bla2@jaylapp.dev".to_string(),
      password: "Password123456".to_string()
    };

    let post_obj_two = PostCreateMember {
      nickname: "BlaName".to_string(),
      mail: "bla3@jaylapp.dev".to_string(),
      password: "Password123456".to_string()
    };

    let _ = account.create(&post_obj);
    let is_err: bool = match account.create(&post_obj_two) {
      Ok(_) => false,
      Err(_) => true
    };
    assert!(is_err);

    account.db_main.execute("DELETE FROM member WHERE mail='bla2@jaylapp.dev'");
    account.db_main.execute("DELETE FROM member WHERE mail='bla3@jaylapp.dev'");
  }

  #[test]
  fn mail_empty() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "NickName".to_string(),
      mail: "".to_string(),
      password: "Password123456".to_string()
    };

    let is_err: bool = match account.create(&post_obj) {
      Ok(_) => false,
      Err(_) => true
    };
    assert!(is_err);
  }

  #[test]
  fn password_empty() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "NickName".to_string(),
      mail: "34234234".to_string(),
      password: "".to_string()
    };

    let is_err: bool = match account.create(&post_obj) {
      Ok(_) => false,
      Err(_) => true
    };
    assert!(is_err);
  }

  #[test]
  fn nickname_empty() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "".to_string(),
      mail: "34234234".to_string(),
      password: "dgsdfsfd".to_string()
    };

    let is_err: bool = match account.create(&post_obj) {
      Ok(_) => false,
      Err(_) => true
    };
    assert!(is_err);
  }

  #[test]
  fn invalid_mail() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "asdasd".to_string(),
      mail: "34234234".to_string(),
      password: "dgsdfsfd".to_string()
    };

    let is_err: bool = match account.create(&post_obj) {
      Ok(_) => false,
      Err(_) => true
    };
    assert!(is_err);
  }

  #[test]
  fn confirm_mail() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "SomeNameWuuh".to_string(),
      mail: "someNameWuuuuh@jaylapp.dev".to_string(),
      password: "Password123456".to_string()
    };

    let account_information = account.create(&post_obj).unwrap();
    let mail_id;
    {
      let member_guard = account.member.read().unwrap();
      let member = member_guard.get(&account_information.id).unwrap();
      mail_id = sha3::hash(vec![&account_information.id.to_string(), &member.salt]);
    }
    account.confirm(&mail_id);
    let confirmed_information = account.get(account_information.id).unwrap();
    assert!(confirmed_information.mail_confirmed);

    account.db_main.execute("DELETE FROM member WHERE mail='someNameWuuuuh@jaylapp.dev'");
  }
}