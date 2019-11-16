extern crate str_util;
extern crate serde;
extern crate serde_json;

use str_util::strformat;
use serde::ser::Serialize;

pub fn expose_api_fn<T: Serialize, U: Serialize>(url: &str, request_type: &str, response_type: &str, schema_response: T, schema_arg1: U) -> serde_json::Value
{
  let template: String = r#"{"url":"{0}","request_type":"{1}","args":[{2}],"response_type":"{3}","response":{4}}"#.to_string();
  serde_json::from_str(&strformat::fmt(template, &[url, request_type, &serde_json::to_string_pretty(&schema_arg1).unwrap(), response_type, &serde_json::to_string_pretty(&schema_response).unwrap()])).unwrap()
}