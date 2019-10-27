use std::collections::HashMap;
use std::sync::RwLock;

pub struct Dictionary {
  pub table: RwLock<HashMap<String, Vec<String>>>
}

impl Default for Dictionary {
  fn default() -> Self
  {
    Dictionary {
      table: RwLock::new(HashMap::new())
    }
  }
}