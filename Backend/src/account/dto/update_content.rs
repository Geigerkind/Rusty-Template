use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Debug, JsonSchema)]
pub struct UpdateContent<C, V> {
  pub content: C,
  pub validation: V,
}