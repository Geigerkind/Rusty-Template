use crate::util::language::material::dictionary::Dictionary;
use crate::util::language::tools::register::Register;
use crate::util::language::domainvalue::language::Language;

pub fn init(dictionary: &Dictionary) {
  dictionary.register("general.service.success", Language::English, "Success!");

  dictionary.register("general.error.validate", Language::English, "TODO");
  dictionary.register("general.error.invalid.mail", Language::English, "TODO");
  dictionary.register("general.error.invalid.nickname", Language::English, "TODO");
  dictionary.register("general.error.invalid.password", Language::English, "TODO");
  dictionary.register("general.error.mail_send", Language::English, "TODO");
  dictionary.register("general.error.unknown", Language::English, "TODO");

  dictionary.register("create.confirmation.subject", Language::English, "TODO");
  dictionary.register("create.confirmation.text", Language::English, "TODO: Heartwarming welcome text\nhttps://jaylapp.dev/API/account/confirm/{0}");
  dictionary.register("create.error.taken.mail", Language::English, "TODO");
  dictionary.register("create.error.taken.nickname", Language::English, "TODO");

  dictionary.register("delete.confirmation.subject", Language::English, "TODO");
  dictionary.register("delete.confirmation.text", Language::English, "TODO: FANCY TEXT\nhttps://jaylapp.dev/API/account/delete/confirm/{0}");

  dictionary.register("forgot.confirmation.subject", Language::English, "TODO");
  dictionary.register("forgot.confirmation.text", Language::English, "TODO: FANCY TEXT\nhttps://jaylapp.dev/API/account/forgot/confirm/{0}");
  dictionary.register("forgot.information.subject", Language::English, "TODO");
  dictionary.register("forgot.information.text", Language::English, "TODO: Text\n New Password: ");
  dictionary.register("forgot.error.no_forgot_issued", Language::English, "TODO");

  dictionary.register("get.error.nomember", Language::English, "TODO");

  dictionary.register("login.error.credentials", Language::English, "TODO");

  dictionary.register("delete.error.no_delete_issued", Language::English, "TODO");
  dictionary.register("delete.error.user_not_removable", Language::English, "TODO");

  dictionary.register("update.error.name_taken", Language::English, "TODO");
  dictionary.register("update.error.mail_taken", Language::English, "TODO");
}