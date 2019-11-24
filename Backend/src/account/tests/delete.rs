#[cfg(test)]
mod tests {
  use mysql_connection::tools::Execute;

  use crate::account::domain_value::CreateMember;
  use crate::account::material::Account;
  use crate::account::tools::{Create, Delete};

  #[test]
  fn issue_delete() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "Nsdsdfsdfsdf".to_string(),
      mail: "hdfgfdgdfd@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj).unwrap();
    let issue_delete = account.issue_delete(val_pair.member_id);
    assert!(issue_delete.is_ok());

    account.db_main.execute("DELETE FROM member WHERE mail='hdfgfdgdfd@jaylappTest.dev'");
  }

  #[test]
  fn confirm_mail() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "hfghsdssdgdfg".to_string(),
      mail: "hfghsdssdgdfg@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj).unwrap();
    let issue_delete = account.issue_delete(val_pair.member_id);
    assert!(issue_delete.is_ok());

    account.db_main.execute("DELETE FROM member WHERE mail='hfghsdssdgdfg@jaylappTest.dev'");
  }
}