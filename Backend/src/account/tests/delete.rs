#[cfg(test)]
mod tests {
  use crate::account::material::account::Account;
  use crate::account::tools::create::Create;
  use crate::account::tools::delete::Delete;
  use crate::account::domainvalue::validation_pair::ValidationPair;
  use crate::account::service::create::PostCreateMember;
  use crate::database::tools::mysql::execute::Execute;

  #[test]
  fn issue_delete_invalid_token() {
    let account = Account::default();
    let val_pair = ValidationPair {
      hash: "someHash".to_string(),
      id: 42
    };
    let issue_delete = account.issue_delete(&val_pair);
    assert!(!issue_delete);
  }

  #[test]
  fn issue_delete() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "Nsdsdfsdfsdf".to_string(),
      mail: "hdfgfdgdfd@jaylappTest.dev".to_string(),
      password: "Password123456".to_string()
    };

    let val_pair = account.create(&post_obj).unwrap();
    let issue_delete = account.issue_delete(&val_pair);
    assert!(issue_delete);

    account.db_main.execute("DELETE FROM member WHERE mail='hdfgfdgdfd@jaylappTest.dev'");
  }

}