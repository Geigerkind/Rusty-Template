use crate::Backend;

use lettre_email::EmailBuilder;
use crate::lettre::Transport;
use lettre::smtp::SmtpClient;
use sha3::{Digest, Sha3_512};

use rand::Rng; 
use rand::distributions::Alphanumeric;
use regex::Regex;

pub trait Util {
    fn send_mail(&self, to: &str, username: &str, subject: &str, text: &str) -> bool;
    fn sha3(&self, input: Vec<&str>) -> String;
    fn random_str(&self, length: usize) -> String;
    fn is_valid_mail(str: &str) -> bool;
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

    fn random_str(&self, length: usize) -> String
    {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .collect::<String>()
    }

    fn is_valid_mail(input: &str) -> bool
    {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([\w-\.]+)@((\[[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.)|(([\w-]+\.)+))([a-zA-Z]{2,4}|[0-9]{1,3})(\]?)$").unwrap();
        }
        RE.is_match(input)
    }
}