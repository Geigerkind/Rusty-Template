use base64;
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::request::{self, FromRequest, Request, State};

use crate::account::Account;
use crate::account::domain_value::ValidationPair;
use crate::account::tools::Token;

pub struct Authenticate;

impl<'a, 'r> FromRequest<'a, 'r> for Authenticate {
  type Error = ();

  fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, ()> {
    let auth_header = req.headers().get_one("Authorization");
    if auth_header.is_none() {
      return Failure((Status::Unauthorized, ()));
    }

    let basic_auth_str = auth_header.unwrap();
    if basic_auth_str.len() <= 5 || !basic_auth_str.contains("Basic ") {
      return Failure((Status::Unauthorized, ()));
    }

    // Base64 encoded id:auth_token
    // We remove "Basic " from the front and feed it to the decode function, as
    // we expect it to be encoded in base64
    let auth_token_res = base64::decode(&basic_auth_str[6..]);
    if auth_token_res.is_err() {
      return Failure((Status::Unauthorized, ()));
    }

    let basic_token_str = String::from_utf8(auth_token_res.unwrap()).unwrap();
    let basic_auth_tokens: Vec<&str> = basic_token_str.split(':').collect();
    if basic_auth_tokens.len() != 2 {
      return Failure((Status::Unauthorized, ()));
    }

    let member_id_res = basic_auth_tokens[0].parse::<u32>();
    if member_id_res.is_err() {
      return Failure((Status::Unauthorized, ()));
    }

    let val_pair = ValidationPair {
      member_id: member_id_res.unwrap(),
      api_token: basic_auth_tokens[1].to_owned(),
    };

    let account = req.guard::<State<'_, Account>>();
    if account.is_failure() || !account.unwrap().validate_token(&val_pair) {
      return Failure((Status::Unauthorized, ()));
    }

    Success(Authenticate)
  }
}