#[cfg(test)]
mod tests {
  use crate::account::material::account::Account;
  use crate::account::tools::validator::Validator;
  use crate::account::domainvalue::validation_pair::ValidationPair;
  use crate::account::tools::create::Create;
  use crate::account::service::create::PostCreateMember;
  use crate::database::tools::mysql::execute::Execute;

  // Helper functions are tested indirectly through all other functions

  #[test]
  fn validate_valid() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "cvcbmnbjfie".to_string(),
      mail: "cvcbmnbjfie@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string()
    };

    let val_pair = account.create(&post_obj).unwrap();
    assert!(account.validate(&val_pair));

    account.db_main.execute("DELETE FROM member WHERE mail='cvcbmnbjfie@jaylappTest.dev'");
  }

  #[test]
  fn validate_invalid() {
    let account = Account::default();
    let val_pair = ValidationPair {
      hash: "someHash".to_string(),
      id: 42
    };

    assert!(!account.validate(&val_pair));
  }
}