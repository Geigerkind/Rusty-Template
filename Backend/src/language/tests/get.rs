#[cfg(test)]
mod tests {
  use crate::language::tools::register::Register;
  use crate::language::tools::get::Get;
  use crate::language::material::dictionary::Dictionary;
  use crate::language::domainvalue::language::Language;

  #[test]
  #[should_panic]
  fn key_not_registered() {
    let dictionary = Dictionary::default();
    dictionary.get("Test", Language::English);
  }

  #[test]
  #[should_panic]
  fn language_not_registered() {
    let dictionary = Dictionary::default();
    dictionary.register("Test", Language::English, "test");
    dictionary.get("Test", Language::German);
  }

  #[test]
  #[should_panic]
  fn language_not_registered_because_empty() {
    let dictionary = Dictionary::default();
    dictionary.register("Test", Language::Japanese, "test");
    dictionary.get("Test", Language::English);
  }

  #[test]
  fn language_exists() {
    let dictionary = Dictionary::default();
    dictionary.register("Test", Language::Japanese, "test");
    assert_eq!(dictionary.get("Test", Language::Japanese), "test")
  }
}