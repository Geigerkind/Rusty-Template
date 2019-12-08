extern crate expose_api;

use expose_api::expose_api_fn;
use rocket_contrib::json::Json;
use schemars::{schema_for, JsonSchema};

use crate::modules::account::dto::{CreateMember, CreateToken, Credentials, Failure, ProlongToken};
use crate::modules::account::domain_value::{AccountInformation};
use crate::modules::account::material::APIToken;

#[derive(JsonSchema)]
struct Nothing;

#[get("/")]
pub fn api() -> Json<Vec<serde_json::Value>> {
  Json(vec![
    // Get
    expose_api_fn("/get/", "get", true, "application/json", schema_for!(Result<AccountInformation, Failure>), schema_for!(Nothing)),

    // Create
    expose_api_fn("/create/<create_member>", "post", false, "application/json", schema_for!(Result<APIToken, Failure>), schema_for!(CreateMember)),
    expose_api_fn("/create/confirm/<confirmation_id>", "get", false, "application/json", schema_for!(bool), schema_for!(String)),
    expose_api_fn("/create/resend/", "get", true, "application/json", schema_for!(bool), schema_for!(Nothing)),

    // Delete
    expose_api_fn("/delete/confirm/<delete_id>", "get", false, "text/html", schema_for!(Result<(), Failure>), schema_for!(String)),
    expose_api_fn("/delete/request/", "delete", true, "application/json", schema_for!(Result<(), Failure>), schema_for!(Nothing)),

    // Forgot
    expose_api_fn("/forgot/confirm/<confirmation_id>", "get", false, "application/json", schema_for!(Result<APIToken, Failure>), schema_for!(String)),
    expose_api_fn("/forgot/send/<mail>", "get", false, "text/html", schema_for!(Result<(), Failure>), schema_for!(String)),

    // Login
    expose_api_fn("/login/<credentials>", "post", false, "application/json", schema_for!(Result<APIToken, Failure>), schema_for!(Credentials)),

    // Update
    expose_api_fn("/update/password/<password>", "post", true, "application/json", schema_for!(Result<APIToken, Failure>), schema_for!(String)),
    expose_api_fn("/update/nickname/<nickname>", "post", true, "application/json", schema_for!(Result<AccountInformation, Failure>), schema_for!(String)),
    expose_api_fn("/update/mail/request/<mail>", "post", true, "application/json", schema_for!(Result<bool, Failure>), schema_for!(String)),
    expose_api_fn("/update/mail/confirm/<confirmation_id>", "get", false, "application/json", schema_for!(Result<APIToken, Failure>), schema_for!(String)),

    // Token
    expose_api_fn("/token/create/<create_token>", "post", true, "application/json", schema_for!(Result<APIToken, Failure>), schema_for!(CreateToken)),
    expose_api_fn("/token/get/", "get", true, "application/json", schema_for!(Result<Vec<APIToken>, Failure>), schema_for!(Nothing)),
    expose_api_fn("/token/delete/<token_id>", "post", true, "application/json", schema_for!(Result<(), Failure>), schema_for!(u32)),
    expose_api_fn("/token/update/<prolong_token>", "post", true, "application/json", schema_for!(Result<APIToken, Failure>), schema_for!(ProlongToken)),
  ])
}