#[cfg(test)]
mod tests {
  use validator;

  #[test]
  fn password_too_short() {
    let pass = "tooshort";
    assert!(validator::password(pass).is_err());
  }

  #[test]
  fn password_has_been_pwned() {
    let pass = "Password123456";
    assert!(validator::password(pass).is_err());
  }

  #[test]
  fn password_is_secure_enough() {
    let pass = "Password123456Password123456Password123456";
    assert!(validator::password(pass).is_ok());
  }

  #[test]
  fn invalid_mail() {
    let mail = "Test@bla";
    assert!(!validator::mail(mail));
  }

  #[test]
  fn valid_mail() {
    let mail = "Test.Test@bla.de";
    assert!(validator::mail(mail));
  }

  #[test]
  fn invalid_nickname() {
    let nickname = "NickName NickName";
    let nickname2 = ".";
    let nickname3 = "@";
    assert!(!validator::nickname(nickname));
    assert!(!validator::nickname(nickname2));
    assert!(!validator::nickname(nickname3));
  }

  #[test]
  fn valid_nickname() {
    let nickname = "NickName";
    assert!(validator::nickname(nickname));
  }
}