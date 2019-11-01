#[cfg(test)]
mod tests {
  use crate::util::validator::tools::valid;

  #[test]
  fn password_too_short() {
    let pass = "tooshort";
    assert!(valid::password(pass).is_err());
  }

  #[test]
  fn password_has_been_pwned() {
    let pass = "Password123456";
    assert!(valid::password(pass).is_err());
  }

  #[test]
  fn password_is_secure_enough() {
    let pass = "Password123456Password123456Password123456";
    assert!(valid::password(pass).is_ok());
  }

  #[test]
  fn invalid_mail() {
    let mail = "Test@bla";
    assert!(!valid::mail(mail));
  }

  #[test]
  fn valid_mail() {
    let mail = "Test.Test@bla.de";
    assert!(valid::mail(mail));
  }

  #[test]
  fn invalid_nickname() {
    let nickname = "NickName NickName";
    assert!(!valid::nickname(nickname));
  }

  #[test]
  fn valid_nickname() {
    let nickname = "NickName";
    assert!(valid::nickname(nickname));
  }
}