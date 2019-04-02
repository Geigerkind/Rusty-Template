use crate::Backend;

use lettre_email::EmailBuilder;
use crate::lettre::Transport;
use lettre::smtp::SmtpClient;
use sha3::{Digest, Sha3_512};

pub trait Util {
    fn send_mail(&self, to: &str, username: &str, subject: &str, text: &str) -> bool;
    fn sha3(&self, input: Vec<&str>) -> String;
}

impl Util for Backend {
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

    fn sha3(&self, input: Vec<&str>) -> String
    {
        let mut hasher = Sha3_512::new();
        hasher.input(input.concat());
        std::str::from_utf8(&hasher.result()).unwrap().to_string()
    }
}