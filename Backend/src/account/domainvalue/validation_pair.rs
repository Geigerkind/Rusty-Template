use schemars::JsonSchema;
#[derive(Deserialize, Serialize, Debug, JsonSchema, Clone)]
pub struct ValidationPair {
    pub api_token: String,
    pub member_id: u32
}
