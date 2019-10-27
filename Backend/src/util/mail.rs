extern crate lettre;
extern crate lettre_email;

use lettre_email::EmailBuilder;
use lettre::Transport;
use lettre::smtp::SmtpClient;

pub fn send(recipient: &str, username: &str, subject: String, text: String) -> bool
{
  let email = EmailBuilder::new()
    .to((recipient, username))
    .from("mail@jaylapp.dev")
    .subject(subject)
    .text(text)
    .build()
    .unwrap().into();

  // Open a local connection on port 25
  let mut mailer = SmtpClient::new_unencrypted_localhost().unwrap().transport();
  let result = mailer.send(email);

  result.is_ok()
}
