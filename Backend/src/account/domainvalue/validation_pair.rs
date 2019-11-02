use schemars::JsonSchema;
#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct ValidationPair {
    pub hash: String,
    pub id: u32
}
