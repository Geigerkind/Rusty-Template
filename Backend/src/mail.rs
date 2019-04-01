use crate::Backend;

use lettre_email::EmailBuilder;
use crate::lettre::Transport;
use lettre::smtp::SmtpClient;

pub trait Mail {
    fn send_mail(&self, to: &str, username: &str, subject: &str, text: &str) -> bool;
}

impl Mail for Backend {
    // This somehow works with postfix
    // DONT TOUCH IT AS LONG AS IT WORKS \(O.O)/
    fn send_mail(&self, to: &str, username: &str, subject: &str, text: &str) -> bool
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
}