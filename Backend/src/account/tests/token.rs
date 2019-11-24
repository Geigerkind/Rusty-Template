#[cfg(test)]
mod tests {
  use mysql_connection::tools::Execute;

  use crate::account::domain_value::{CreateMember, Credentials, ValidationPair};
  use crate::account::material::Account;
  use crate::account::tools::{Create, Login, Token, Update};

  #[test]
  fn validate_valid() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "cvcbmnbjfie".to_string(),
      mail: "cvcbmnbjfie@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj).unwrap();
    assert!(account.validate_token(&val_pair));

    account.db_main.execute("DELETE FROM member WHERE mail='cvcbmnbjfie@jaylappTest.dev'");
  }

  #[test]
  fn validate_invalid() {
    let account = Account::default();
    let val_pair = ValidationPair {
      api_token: "someHash".to_string(),
      member_id: 42,
    };

    assert!(!account.validate_token(&val_pair));
  }

  #[test]
  fn validation_invalid_after_update() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "klsdkfsowerf".to_string(),
      mail: "klsdkfsowerf@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    // First login
    let val_pair = account.create(&post_obj).unwrap();
    let val_pair2 = account.login(&Credentials {
      mail: post_obj.mail,
      password: post_obj.password,
    }).unwrap();
    assert!(account.validate_token(&val_pair));
    assert!(account.validate_token(&val_pair2));

    let val_pair3 = account.change_password("SuperDuperSecretPasswordDefNotSecretTho", val_pair.member_id).unwrap();
    assert!(!account.validate_token(&val_pair2));
    assert!(account.validate_token(&val_pair3));

    account.db_main.execute("DELETE FROM member WHERE mail='klsdkfsowerf@jaylappTest.dev'");
  }

  #[test]
  fn get_all_tokens() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "fhfgjhfgjfghfjg".to_string(),
      mail: "fhfgjhfgjfghfjg@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };
    let val_pair = account.create(&post_obj).unwrap();
    let tokens = account.get_all_token(val_pair.member_id);
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token, val_pair.api_token);

    account.db_main.execute("DELETE FROM member WHERE mail='fhfgjhfgjfghfjg@jaylappTest.dev'");
  }

  #[test]
  fn delete_token() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "sadgsdfgsddfgsdg".to_string(),
      mail: "sadgsdfgsddfgsdg@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };
    let val_pair = account.create(&post_obj).unwrap();
    assert!(account.validate_token(&val_pair));

    let new_token_res = account.create_token("Login", val_pair.member_id, 42);
    assert!(new_token_res.is_ok());
    let new_token = new_token_res.unwrap();
    assert!(account.validate_token(&new_token.to_validation_pair()));

    assert!(account.delete_token(new_token.id, val_pair.member_id).is_ok());
    assert!(!account.validate_token(&new_token.to_validation_pair()));

    account.db_main.execute("DELETE FROM member WHERE mail='sadgsdfgsddfgsdg@jaylappTest.dev'");
  }
}