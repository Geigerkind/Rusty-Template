#[cfg(test)]
mod tests {
  use mysql_connection::tools::Execute;

  use crate::account::domain_value::CreateMember;
  use crate::account::material::Account;
  use crate::account::tools::{Create, GetAccountInformation};

  #[test]
  fn get_does_not_exist() {
    let account = Account::default();
    let acc_info = account.get(42);
    assert!(acc_info.is_err());
  }

  #[test]
  fn get_exists() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "ijfeuhifsduhisdfuhiuhisdf".to_string(),
      mail: "ijfeuhifsduhisdfuhiuhisdf@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let login = account.create(&post_obj).unwrap();
    let acc_info = account.get(login.member_id);
    assert!(acc_info.is_ok());

    account.db_main.execute("DELETE FROM member WHERE mail='ijfeuhifsduhisdfuhiuhisdf@jaylappTest.dev'");
  }
}