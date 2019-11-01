#[cfg(test)]
mod tests {
  use crate::account::material::account::Account;
  use crate::account::tools::create::Create;
  use crate::account::tools::delete::Delete;
  use crate::account::domainvalue::validation_pair::ValidationPair;
  use crate::account::domainvalue::post_create_member::PostCreateMember;
  use crate::database::tools::mysql::execute::Execute;

  #[test]
  fn issue_delete_invalid_token() {
    let account = Account::default();
    let val_pair = ValidationPair {
      hash: "someHash".to_string(),
      id: 42
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
      password: "Password123456Password123456Password123456".to_string()
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
      password: "Password123456Password123456Password123456".to_string()
    };

    let val_pair = account.create(&post_obj).unwrap();
    let issue_delete = account.issue_delete(&val_pair);
    assert!(issue_delete.is_ok());

    account.db_main.execute("DELETE FROM member WHERE mail='hfghsdssdgdfg@jaylappTest.dev'");
  }

}