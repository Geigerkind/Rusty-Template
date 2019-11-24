#[cfg(test)]
mod tests {
  use mysql_connection::tools::Execute;

  use crate::account::domain_value::{PostCreateMember, ValidationPair};
  use crate::account::material::Account;
  use crate::account::tools::{Create, Delete};

  #[test]
  fn issue_delete_invalid_token() {
    let account = Account::default();
    let val_pair = ValidationPair {
      api_token: "someHash".to_string(),
      member_id: 42,
    };
    let issue_delete = account.issue_delete(&val_pair);
    assert!(issue_delete.is_err());
  }

  #[test]
  fn issue_delete() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "Nsdsdfsdfsdf".to_string(),
      mail: "hdfgfdgdfd@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj).unwrap();
    let issue_delete = account.issue_delete(&val_pair);
    assert!(issue_delete.is_ok());

    account.db_main.execute("DELETE FROM member WHERE mail='hdfgfdgdfd@jaylappTest.dev'");
  }

  #[test]
  fn confirm_mail() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "hfghsdssdgdfg".to_string(),
      mail: "hfghsdssdgdfg@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj).unwrap();
    let issue_delete = account.issue_delete(&val_pair);
    assert!(issue_delete.is_ok());

    account.db_main.execute("DELETE FROM member WHERE mail='hfghsdssdgdfg@jaylappTest.dev'");
  }
}