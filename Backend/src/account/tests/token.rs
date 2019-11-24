#[cfg(test)]
mod tests {
  use mysql_connection::tools::Execute;

  use crate::account::domain_value::{PostCreateMember, PostDeleteToken, PostLogin, PostToken, ValidationPair};
  use crate::account::material::{Account, PostChangeStr};
  use crate::account::tools::{Create, Login, Token, Update};

  #[test]
  fn validate_valid() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "cvcbmnbjfie".to_string(),
      mail: "cvcbmnbjfie@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj).unwrap();
    assert!(account.validate(&val_pair));

    account.db_main.execute("DELETE FROM member WHERE mail='cvcbmnbjfie@jaylappTest.dev'");
  }

  #[test]
  fn validate_invalid() {
    let account = Account::default();
    let val_pair = ValidationPair {
      api_token: "someHash".to_string(),
      member_id: 42,
    };

    assert!(!account.validate(&val_pair));
  }

  #[test]
  fn validation_invalid_after_update() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "klsdkfsowerf".to_string(),
      mail: "klsdkfsowerf@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    // First login
    let val_pair = account.create(&post_obj).unwrap();
    let val_pair2 = account.login(&PostLogin {
      mail: post_obj.mail,
      password: post_obj.password,
    }).unwrap();

    assert!(account.validate(&val_pair));
    assert!(account.validate(&val_pair2));

    let val_pair3 = account.change_password(&PostChangeStr {
      content: "SuperDuperSecretPasswordDefNotSecretTho".to_string(),
      validation: val_pair,
    }).unwrap();

    assert!(!account.validate(&val_pair2));
    assert!(account.validate(&val_pair3));

    account.db_main.execute("DELETE FROM member WHERE mail='klsdkfsowerf@jaylappTest.dev'");
  }

  #[test]
  fn get_all_tokens() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "fhfgjhfgjfghfjg".to_string(),
      mail: "fhfgjhfgjfghfjg@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };
    let val_pair = account.create(&post_obj).unwrap();
    let token_res = account.get_all_token(&val_pair);
    assert!(token_res.is_ok());
    let tokens = token_res.unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].token, val_pair.api_token);

    account.db_main.execute("DELETE FROM member WHERE mail='fhfgjhfgjfghfjg@jaylappTest.dev'");
  }

  #[test]
  fn delete_token() {
    let account = Account::default();
    let post_obj = PostCreateMember {
      nickname: "sadgsdfgsddfgsdg".to_string(),
      mail: "sadgsdfgsddfgsdg@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };
    let val_pair = account.create(&post_obj).unwrap();
    assert!(account.validate(&val_pair));

    let new_token_res = account.create_token(&PostToken {
      purpose: "Login".to_string(),
      exp_date: 42,
      val_pair: val_pair.clone(),
    });
    assert!(new_token_res.is_ok());
    let new_token = new_token_res.unwrap();
    assert!(account.validate(&new_token.to_validation_pair()));

    assert!(account.delete_token(&PostDeleteToken {
      token_id: new_token.id,
      val_pair: val_pair.clone(),
    }).is_ok());
    assert!(!account.validate(&new_token.to_validation_pair()));

    account.db_main.execute("DELETE FROM member WHERE mail='sadgsdfgsddfgsdg@jaylappTest.dev'");
  }
}