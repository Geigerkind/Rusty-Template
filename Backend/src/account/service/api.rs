extern crate expose_api;

use expose_api::expose_api_fn;
use rocket_contrib::json::Json;
use schemars::schema_for;

use crate::account::domain_value::{AccountInformation, PostCreateMember, PostDeleteToken, PostLogin, PostToken, ValidationPair};
use crate::account::material::{APIToken, PostChangeStr, PostChangeStrLogin};

#[get("/")]
pub fn api() -> Json<Vec<serde_json::Value>> {
  Json(vec![
    // Get
    expose_api_fn("/get/<id>", "get", "application/json", schema_for!(Result<AccountInformation, String>), schema_for!(u32)),

    // Create
    expose_api_fn("/create/send/<params>", "post", "application/json", schema_for!(Result<ValidationPair, String>), schema_for!(PostCreateMember)),
    expose_api_fn("/create/confirm/<id>", "get", "application/json", schema_for!(bool), schema_for!(String)),
    expose_api_fn("/create/resend/<params>", "post", "application/json", schema_for!(bool), schema_for!(ValidationPair)),

    // Delete
    expose_api_fn("/delete/confirm/<id>", "get", "text/html", schema_for!(String), schema_for!(String)),
    expose_api_fn("/delete/send/<params>", "post", "application/json", schema_for!(Result<AccountInformation, String>), schema_for!(ValidationPair)),

    // Forgot
    expose_api_fn("/forgot/confirm/<id>", "get", "application/json", schema_for!(Result<ValidationPair, String>), schema_for!(String)),
    expose_api_fn("/forgot/send/<mail>", "get", "text/html", schema_for!(String), schema_for!(String)),

    // Login
    expose_api_fn("/login/<params>", "post", "application/json", schema_for!(Result<ValidationPair, String>), schema_for!(PostLogin)),

    // Update
    expose_api_fn("/update/password/<params>", "post", "application/json", schema_for!(Result<ValidationPair, String>), schema_for!(PostChangeStr)),
    expose_api_fn("/update/nickname/<params>", "post", "application/json", schema_for!(Result<AccountInformation, String>), schema_for!(PostChangeStr)),
    expose_api_fn("/update/mail/<params>", "post", "application/json", schema_for!(Result<ValidationPair, String>), schema_for!(PostChangeStrLogin)),

    // Token
    expose_api_fn("/token/create/<params>", "post", "application/json", schema_for!(Result<APIToken, String>), schema_for!(PostToken)),
    // Not the truth, but close enough
    expose_api_fn("/token/get/<params>", "post", "application/json", schema_for!(Result<Vec<APIToken>, String>), schema_for!(ValidationPair)),
    expose_api_fn("/token/delete/<params>", "post", "application/json", schema_for!(Result<(), String>), schema_for!(PostDeleteToken)),
  ])
}