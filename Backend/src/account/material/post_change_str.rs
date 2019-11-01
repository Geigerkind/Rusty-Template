use crate::account::domainvalue::validation_pair::ValidationPair;

#[derive(Deserialize)]
pub struct PostChangeStr {
  pub content: String,
  pub validation: ValidationPair
}