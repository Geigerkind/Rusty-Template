#[cfg(test)]
mod tests {
  use mysql_connection::tools::Execute;

  use crate::account::domain_value::{CreateMember, Credentials, ValidationPair, UpdateContent, UpdateContentCredentials};
  use crate::account::material::Account;
  use crate::account::tools::{Create, Update};

  #[test]
  fn change_name() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "ijofsdiojsdfgiuhig".to_string(),
      mail: "ijofsdiojsdfgiuhig@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj).unwrap();
    let changed_name = account.change_name(&UpdateContent {
      content: "SomeUsername".to_string(),
      validation: val_pair,
    });
    assert!(changed_name.is_ok());
    assert_eq!(changed_name.unwrap().nickname, "SomeUsername".to_string());

    account.db_main.execute("DELETE FROM member WHERE mail='ijofsdiojsdfgiuhig@jaylappTest.dev'");
  }

  #[test]
  fn change_name_invalid_validation() {
    let account = Account::default();
    let val_pair = ValidationPair {
      api_token: "someHash".to_string(),
      member_id: 42,
    };

    let changed_name = account.change_name(&UpdateContent {
      content: "Some Username".to_string(),
      validation: val_pair,
    });

    assert!(changed_name.is_err());
  }

  #[test]
  fn change_name_empty_content() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "siodjfijsiojiospq".to_string(),
      mail: "siodjfijsiojiospq@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj).unwrap();
    let changed_name = account.change_name(&UpdateContent {
      content: "".to_string(),
      validation: val_pair,
    });
    assert!(changed_name.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='siodjfijsiojiospq@jaylappTest.dev'");
  }

  #[test]
  fn change_name_invalid_content() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "ihsdfoiosdf".to_string(),
      mail: "ihsdfoiosdf@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj).unwrap();
    let changed_name = account.change_name(&UpdateContent {
      content: "ihsdfoiosdf ihsdfoiosdf".to_string(),
      validation: val_pair,
    });
    println!("{:?}", changed_name);
    assert!(changed_name.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='ihsdfoiosdf@jaylappTest.dev'");
  }

  #[test]
  fn change_name_name_taken() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "oasijidhaais".to_string(),
      mail: "oasijidhaais@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let post_obj_two = CreateMember {
      nickname: "guhzasooas".to_string(),
      mail: "guhzasooas@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj).unwrap();
    let _ = account.create(&post_obj_two).unwrap();
    let changed_name = account.change_name(&UpdateContent {
      content: post_obj_two.nickname,
      validation: val_pair,
    });
    assert!(changed_name.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='oasijidhaais@jaylappTest.dev'");
    account.db_main.execute("DELETE FROM member WHERE mail='guhzasooas@jaylappTest.dev'");
  }

  #[test]
  fn change_password_invalid_validation() {
    let account = Account::default();
    let val_pair = ValidationPair {
      api_token: "someHash".to_string(),
      member_id: 42,
    };

    let changed_password = account.change_password(&UpdateContent {
      content: "Some Username".to_string(),
      validation: val_pair,
    });

    assert!(changed_password.is_err());
  }

  #[test]
  fn change_password_empty_content() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "mvfhhbvidsd".to_string(),
      mail: "mvfhhbvidsd@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj).unwrap();
    let changed_password = account.change_password(&UpdateContent {
      content: "".to_string(),
      validation: val_pair,
    });
    assert!(changed_password.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='mvfhhbvidsd@jaylappTest.dev'");
  }

  #[test]
  fn change_password() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "xdsdfgsdgs".to_string(),
      mail: "xdsdfgsdgs@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj).unwrap();
    let val_pair_hash = val_pair.api_token.clone();
    let val_pair_id = val_pair.member_id;
    let changed_password = account.change_password(&UpdateContent {
      content: "SomeWeirdPassword".to_string(),
      validation: val_pair,
    });
    assert!(changed_password.is_ok());
    let new_val_pair = changed_password.unwrap();
    assert_ne!(new_val_pair.api_token, val_pair_hash);
    assert_eq!(new_val_pair.member_id, val_pair_id);

    account.db_main.execute("DELETE FROM member WHERE mail='xdsdfgsdgs@jaylappTest.dev'");
  }

  #[test]
  fn change_mail_invalid_validation() {
    let account = Account::default();
    let credentials = Credentials {
      mail: "bla@bla.de".to_string(),
      password: "somepassword".to_string(),
    };

    let changed_mail = account.change_mail(&UpdateContentCredentials {
      content: "Some Username".to_string(),
      credentials,
    });

    assert!(changed_mail.is_err());
  }

  #[test]
  fn change_mail_empty_content() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "nsigsvbsdsd".to_string(),
      mail: "nsigsvbsdsd@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };
    let credentials = Credentials {
      mail: "nsigsvbsdsd@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let _ = account.create(&post_obj).unwrap();
    let changed_mail = account.change_mail(&UpdateContentCredentials {
      content: "".to_string(),
      credentials,
    });
    assert!(changed_mail.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='nsigsvbsdsd@jaylappTest.dev'");
  }

  #[test]
  fn change_mail_invalid_content() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "asiudfuhisduifs".to_string(),
      mail: "asiudfuhisduifs@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };
    let credentials = Credentials {
      mail: "asiudfuhisduifs@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let _ = account.create(&post_obj).unwrap();
    let changed_mail = account.change_mail(&UpdateContentCredentials {
      content: "asiudfuhisduifs".to_string(),
      credentials,
    });
    assert!(changed_mail.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='asiudfuhisduifs@jaylappTest.dev'");
  }

  #[test]
  fn change_mail_mail_taken() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "csdazgtsdczas".to_string(),
      mail: "csdazgtsdczas@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };
    let credentials = Credentials {
      mail: "csdazgtsdczas@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let post_obj_two = CreateMember {
      nickname: "bdvshudvbsdv".to_string(),
      mail: "bdvshudvbsdv@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let _ = account.create(&post_obj).unwrap();
    let _ = account.create(&post_obj_two).unwrap();
    let changed_mail = account.change_mail(&UpdateContentCredentials {
      content: post_obj_two.mail,
      credentials,
    });
    assert!(changed_mail.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='csdazgtsdczas@jaylappTest.dev'");
    account.db_main.execute("DELETE FROM member WHERE mail='bdvshudvbsdv@jaylappTest.dev'");
  }

  #[test]
  fn change_mail() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "xdssdfsdfg".to_string(),
      mail: "xdssdfsdfg@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };
    let credentials = Credentials {
      mail: "xdssdfsdfg@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj).unwrap();
    let val_pair_hash = val_pair.api_token.clone();
    let val_pair_id = val_pair.member_id;
    let changed_mail = account.change_mail(&UpdateContentCredentials {
      content: "xdssdfsdfg2@bla.de".to_string(),
      credentials,
    });
    assert!(changed_mail.is_ok());
    let new_val_pair = changed_mail.unwrap();
    assert_ne!(new_val_pair.api_token, val_pair_hash);
    assert_eq!(new_val_pair.member_id, val_pair_id);

    account.db_main.execute("DELETE FROM member WHERE mail='xdssdfsdfg2@bla.de'");
  }
}