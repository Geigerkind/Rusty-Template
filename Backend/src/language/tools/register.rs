use crate::language::domainvalue::language::Language;
use crate::language::material::dictionary::Dictionary;

pub trait Register {
  fn register(&self, key: &str, language: Language, value: &str);
}

impl Register for Dictionary {
  fn register(&self, key: &str, language: Language, value: &str) {
    let mut lang_table = self.table.write().unwrap();
    let key_str: String = String::from(key);
    let value_str: String = String::from(value);
    let lang_index = language as usize;
    match lang_table.get_mut(&key_str) {
      Some(vec) => {
        let vec_length = vec.len();
        match vec.get_mut(lang_index) {
          Some(content) => {
            if content.is_empty() {
              *content = value_str;
            } else {
              panic!("{} is overwritten for the language {} with the content {}!", key, lang_index, value);
            }
          },
          // This means that the vector has not been extended to this language yet
          None => {
            for _ in {vec_length..lang_index} {
              vec.push(String::new());
            }
            vec.push(value_str);
          }
        }
      },
      None => {
        let mut container: Vec<String> = Vec::new();
        for _ in {0..lang_index} {
          container.push(String::new());
        }
        container.push(value_str);
        lang_table.insert(key_str, container);
      }
    }
  }
}