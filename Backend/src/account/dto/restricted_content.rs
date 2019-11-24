use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct RestrictedContent<C, V> {
  pub content: C,
  pub validation: V,
}