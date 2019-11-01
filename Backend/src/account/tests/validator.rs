#[cfg(test)]
mod tests {
  use crate::account::material::account::Account;
  use crate::account::tools::validator::Validator;
  use crate::account::domainvalue::validation_pair::ValidationPair;
  use crate::account::tools::create::Create;
  use crate::account::service::create::PostCreateMember;
  use crate::database::tools::mysql::execute::Execute;
  use crate::account::tools::update::Update;
  use crate::account::service::update::PostChangeStr;
  use crate::account::tools::login::Login;
  use crate::account::service::login::PostLogin;

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

  #[test]
  fn validation_invalid_after_update() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "klsdkfsowerf".to_string(),
      mail: "klsdkfsowerf@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string()
    };

    // First login
    let val_pair = account.create(&post_obj).unwrap();
    let val_pair2 = account.login(&PostLogin {
      mail: post_obj.mail,
      password: post_obj.password
    }).unwrap();

    assert!(account.validate(&val_pair));
    assert!(account.validate(&val_pair2));

    let val_pair3 = account.change_password(&PostChangeStr {
      content: "SuperDuperSecretPasswordDefNotSecretTho".to_string(),
      validation: val_pair
    }).unwrap();

    assert!(!account.validate(&val_pair2));
    assert!(account.validate(&val_pair3));

    account.db_main.execute("DELETE FROM member WHERE mail='klsdkfsowerf@jaylappTest.dev'");
  }
}