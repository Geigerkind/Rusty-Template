#[cfg(test)]
mod tests {
  use crate::account::material::account::Account;
  use crate::account::tools::forgot::Forgot;
  use crate::account::tools::create::Create;
  use crate::account::domainvalue::post_create_member::PostCreateMember;
  use mysql_connection::tools::Execute;
  use str_util::sha3;

  #[test]
  fn send_forget_password_user_does_not_exist() {
    let account = Account::default();
    assert!(account.send_forgot_password("test@mail.de").is_ok());
  }

  #[test]
  fn send_forget_password_invalid_mail() {
    let account = Account::default();
    assert!(account.send_forgot_password("test").is_err());
  }

  #[test]
  fn send_forgot_password_user_exists_and_receive() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "fscngsuzfdcsv".to_string(),
      mail: "fscngsuzfdcsv@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string()
    };

    let val_pair = account.create(&post_obj).unwrap();
    assert!(account.send_forgot_password("fscngsuzfdcsv@jaylappTest.dev").is_ok());

    let salt;
    {
      let member = account.member.read().unwrap();
      salt = member.get(&val_pair.member_id).unwrap().salt.clone();
    }
    let forgot_id = sha3::hash(&[&val_pair.member_id.to_string(), "forgot", &salt]);
    let receive_forgot = account.recv_forgot_password(&forgot_id);
    println!("{:?}", receive_forgot);
    assert!(receive_forgot.is_ok());

    account.db_main.execute("DELETE FROM member WHERE mail='fscngsuzfdcsv@jaylappTest.dev'");
  }

  #[test]
  fn recv_forgot_password_invalid_id() {
    let account = Account::default();
    assert!(account.recv_forgot_password("bla").is_err());
  }

}