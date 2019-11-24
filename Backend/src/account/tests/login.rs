#[cfg(test)]
mod tests {
  use crate::account::domain_value::PostLogin;
  use crate::account::material::Account;
  use crate::account::tools::Login;

  // User exists login is tested when creating an account
  #[test]
  fn login_user_does_not_exist() {
    let account = Account::default();
    let credentials = PostLogin {
      mail: "NothingLol".to_string(),
      password: "NotSecret".to_string(),
    };
    let login = account.login(&credentials);
    assert!(login.is_err());
  }
}