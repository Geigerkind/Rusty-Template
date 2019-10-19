extern crate lettre;
extern crate lettre_email;

use lettre_email::EmailBuilder;
use lettre::Transport;
use lettre::smtp::SmtpClient;

pub fn send_mail(to: &str, username: &str, subject: &str, text: &str) -> bool
{
  let email = EmailBuilder::new()
    .to((to, username))
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
