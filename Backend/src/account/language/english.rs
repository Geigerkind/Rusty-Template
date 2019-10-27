use crate::language::material::dictionary::Dictionary;
use crate::language::tools::register::Register;
use crate::language::domainvalue::language::Language;

pub fn init(dictionary: &Dictionary) {
  dictionary.register("create.confirmation.subject", Language::English, "TODO");
  dictionary.register("create.confirmation.text", Language::English, "TODO: Heartwarming welcome text\nhttps://jaylapp.dev/API/account/confirm/{0}");
  dictionary.register("create.error.empty.nickname", Language::English, "TODO");
  dictionary.register("create.error.empty.mail", Language::English, "TODO");
  dictionary.register("create.error.empty.password", Language::English, "TODO");
  dictionary.register("create.error.taken.mail", Language::English, "TODO");
  dictionary.register("create.error.taken.nickname", Language::English, "TODO");
  dictionary.register("create.error.invalid.mail", Language::English, "TODO");
  dictionary.register("create.error.unknown", Language::English, "TODO");

  dictionary.register("delete.confirmation.subject", Language::English, "TODO");
  dictionary.register("delete.confirmation.text", Language::English, "TODO: FANCY TEXT\nhttps://jaylapp.dev/API/account/delete/confirm/{0}");

  dictionary.register("forgot.confirmation.subject", Language::English, "TODO");
  dictionary.register("forgot.confirmation.text", Language::English, "TODO: FANCY TEXT\nhttps://jaylapp.dev/API/account/forgot/confirm/{0}");
  dictionary.register("forgot.information.subject", Language::English, "TODO");
  dictionary.register("forgot.information.text", Language::English, "TODO: Text\n New Password: ");
}