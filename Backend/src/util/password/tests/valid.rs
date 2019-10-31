#[cfg(test)]
mod tests {
  use crate::util::password::tools::valid;

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

}