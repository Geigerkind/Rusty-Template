#[cfg(test)]
mod tests {
  use mysql_connection::tools::Execute;

  use crate::account::dto::CreateMember;
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

    let val_pair = account.create(&post_obj.mail, &post_obj.nickname, &post_obj.password).unwrap();
    let changed_name = account.change_name("SomeUsername", val_pair.member_id);
    assert!(changed_name.is_ok());
    assert_eq!(changed_name.unwrap().nickname, "SomeUsername".to_string());

    account.db_main.execute("DELETE FROM member WHERE mail='ijofsdiojsdfgiuhig@jaylappTest.dev'");
  }

  #[test]
  fn change_name_empty_content() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "siodjfijsiojiospq".to_string(),
      mail: "siodjfijsiojiospq@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj.mail, &post_obj.nickname, &post_obj.password).unwrap();
    let changed_name = account.change_name("", val_pair.member_id);
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

    let val_pair = account.create(&post_obj.mail, &post_obj.nickname, &post_obj.password).unwrap();
    let changed_name = account.change_name("ihsdfoiosdf ihsdfoiosdf", val_pair.member_id);
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

    let val_pair = account.create(&post_obj.mail, &post_obj.nickname, &post_obj.password).unwrap();
    let _ = account.create(&post_obj_two.mail, &post_obj_two.nickname, &post_obj_two.password).unwrap();
    let changed_name = account.change_name(&post_obj_two.nickname, val_pair.member_id);
    assert!(changed_name.is_err());

    account.db_main.execute("DELETE FROM member WHERE mail='oasijidhaais@jaylappTest.dev'");
    account.db_main.execute("DELETE FROM member WHERE mail='guhzasooas@jaylappTest.dev'");
  }

  #[test]
  fn change_password_empty_content() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "mvfhhbvidsd".to_string(),
      mail: "mvfhhbvidsd@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj.mail, &post_obj.nickname, &post_obj.password).unwrap();
    let changed_password = account.change_password("", val_pair.member_id);
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

    let val_pair = account.create(&post_obj.mail, &post_obj.nickname, &post_obj.password).unwrap();
    let val_pair_hash = val_pair.api_token.clone();
    let val_pair_id = val_pair.member_id;
    let changed_password = account.change_password("SomeWeirdPassword", val_pair.member_id);
    assert!(changed_password.is_ok());
    let new_val_pair = changed_password.unwrap();
    assert_ne!(new_val_pair.api_token, val_pair_hash);
    assert_eq!(new_val_pair.member_id, val_pair_id);

    account.db_main.execute("DELETE FROM member WHERE mail='xdsdfgsdgs@jaylappTest.dev'");
  }

  #[test]
  fn change_mail_empty_content() {
    let account = Account::default();
    let post_obj = CreateMember {
      nickname: "nsigsvbsdsd".to_string(),
      mail: "nsigsvbsdsd@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj.mail, &post_obj.nickname, &post_obj.password).unwrap();
    let changed_mail = account.change_mail("", val_pair.member_id);
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

    let val_pair = account.create(&post_obj.mail, &post_obj.nickname, &post_obj.password).unwrap();
    let changed_mail = account.change_mail("asiudfuhisduifs", val_pair.member_id);
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

    let post_obj_two = CreateMember {
      nickname: "bdvshudvbsdv".to_string(),
      mail: "bdvshudvbsdv@jaylappTest.dev".to_string(),
      password: "Password123456Password123456Password123456".to_string(),
    };

    let val_pair = account.create(&post_obj.mail, &post_obj.nickname, &post_obj.password).unwrap();
    let _ = account.create(&post_obj_two.mail, &post_obj_two.nickname, &post_obj_two.password).unwrap();
    let changed_mail = account.change_mail(&post_obj_two.mail, val_pair.member_id);
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

    let val_pair = account.create(&post_obj.mail, &post_obj.nickname, &post_obj.password).unwrap();
    let val_pair_hash = val_pair.api_token.clone();
    let val_pair_id = val_pair.member_id;
    let changed_mail = account.change_mail("xdssdfsdfg2@bla.de", val_pair_id);
    assert!(changed_mail.is_ok());
    let new_val_pair = changed_mail.unwrap();
    assert_ne!(new_val_pair.api_token, val_pair_hash);
    assert_eq!(new_val_pair.member_id, val_pair_id);

    account.db_main.execute("DELETE FROM member WHERE mail='xdssdfsdfg2@bla.de'");
  }
}