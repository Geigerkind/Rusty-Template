#[cfg(test)]
mod tests {
  use crate::account::material::account::Account;
  use crate::account::tools::create::Create;
  use crate::account::domainvalue::post_create_member::PostCreateMember;
  use crate::database::tools::mysql::execute::Execute;
  use crate::account::tools::get::GetAccountInformation;

  #[test]
  fn get_does_not_exist() {
    let account = Account::default();
    let acc_info = account.get(42);
    assert!(acc_info.is_err());
  }

  #[test]
  fn get_exists() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "ijfeuhifsduhisdfuhiuhisdf".to_string(),
      mail: "ijfeuhifsduhisdfuhiuhisdf@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string()
    };

    let login = account.create(&post_obj).unwrap();
    let acc_info = account.get(login.member_id);
    assert!(acc_info.is_ok());

    account.db_main.execute("DELETE FROM member WHERE mail='ijfeuhifsduhisdfuhiuhisdf@jaylappTest.dev'");
  }
}