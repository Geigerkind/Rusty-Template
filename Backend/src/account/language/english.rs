use language::domain_value::Language;
use language::material::Dictionary;
use language::tools::Register;

pub fn init(dictionary: &Dictionary) {
  dictionary.register("general.error.password.pwned", Language::English, "This password has been pwned {0} times. Please choose another password!");
  dictionary.register("general.error.password.length", Language::English, "The minimum length for a password is 12 character.");

  dictionary.register("general.service.success", Language::English, "Success!");
  dictionary.register("general.login", Language::English, "LogIn");

  dictionary.register("general.error.validate", Language::English, "Invalid credentials!");
  dictionary.register("general.error.invalid.mail", Language::English, "Invalid mail!");
  dictionary.register("general.error.invalid.nickname", Language::English, "Invalid nickname!");
  dictionary.register("general.error.unknown", Language::English, "An unknown error occurred!");

  dictionary.register("create.confirmation.subject", Language::English, "Confirm your account!");
  dictionary.register("create.confirmation.text", Language::English, "Greetings!\n\nPlease finish the registration process by clicking on the provided url.\n\nhttps://jaylapp.dev/API/account/confirm/{0}\n\nCheers!");
  dictionary.register("create.error.taken.mail", Language::English, "This mail is already being used!");
  dictionary.register("create.error.taken.nickname", Language::English, "This nickname is already being used!");

  dictionary.register("forgot.confirmation.subject", Language::English, "Have you forgotten your password?");
  dictionary.register("forgot.confirmation.text", Language::English, "Greetings!\nPlease click on the provided url in order to generate a new password.\n\nhttps://jaylapp.dev/API/account/forgot/confirm/{0}\n\nCheers!");
  dictionary.register("forgot.information.subject", Language::English, "Your new password!");
  dictionary.register("forgot.information.text", Language::English, "Greetings\n\nThis is your new Password: {0}\n\nPlease change it immediately!\n\nCheers!");
  dictionary.register("forgot.error.no_forgot_issued", Language::English, "The provided url is invalid!");

  dictionary.register("get.error.nomember", Language::English, "There is no member with this id!");

  dictionary.register("login.error.credentials", Language::English, "Invalid credentials!");

  dictionary.register("delete.error.no_delete_issued", Language::English, "A deletion has not been requested for this token!");
  dictionary.register("delete.error.user_not_removable", Language::English, "Unable to delete user!");
  dictionary.register("delete.confirmation.subject", Language::English, "Confirm the deletion of your account!");
  dictionary.register("delete.confirmation.text", Language::English, "Greetings!\n\nPlease confirm the deletion of your account by clicking on the provided url.\n\nhttps://jaylapp.dev/API/account/delete/confirm/{0}\n\nCheers!");

  dictionary.register("update.error.name_taken", Language::English, "This nickname is already taken!");
  dictionary.register("update.error.mail_taken", Language::English, "This mail is already taken");
}