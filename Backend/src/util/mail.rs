extern crate lettre;
extern crate lettre_email;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use lettre_email::EmailBuilder;
use lettre::Transport;
use lettre::ClientSecurity;
use lettre::smtp::SmtpClient;

pub fn send(recipient: &str, username: &str, subject: String, text: String) -> bool
{
  dotenv().ok();

  let email = EmailBuilder::new()
    .to((recipient, username))
    .from("jaylappdev@gmail.com")
    .subject(subject)
    .text(text)
    .build()
    .unwrap().into();

  // Open a local connection on port 25
  let mut mailer = SmtpClient::new(format!("127.0.0.1:{}", env::var("SMTP_PORT").unwrap()), ClientSecurity::None).unwrap().transport();
  let result = mailer.send(email);

  println!("{:?}", result);
  result.is_ok()
}
