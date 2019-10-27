use crate::language::material::dictionary::Dictionary;
use crate::language::tools::register::Register;
use crate::language::domainvalue::language::Language;

pub fn init(dictionary: &Dictionary) {
  dictionary.register("create.confirmation.subject", Language::English, "TODO");
  dictionary.register("create.confirmation.text", Language::English, "TODO: Heartwarming welcome text\nhttps://jaylapp.dev/API/account/confirm/");

  dictionary.register("delete.confirmation.subject", Language::English, "TODO");
  dictionary.register("delete.confirmation.text", Language::English, "TODO: FANCY TEXT\nhttps://jaylapp.dev/API/account/delete/confirm/");

  dictionary.register("forgot.confirmation.subject", Language::English, "TODO");
  dictionary.register("forgot.confirmation.text", Language::English, "TODO: FANCY TEXT\nhttps://jaylapp.dev/API/account/forgot/confirm/");
  dictionary.register("forgot.information.subject", Language::English, "TODO");
  dictionary.register("forgot.information.text", Language::English, "TODO: Text\n New Password: ");
}