import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { LoginRoutingModule } from "./routing.module";
import { Login } from "./login";
import { GeneralInputModule } from "../template/general_input/general_input.module";
import { PasswordInputModule } from "../template/password_input/password_input.module";
import { ConfirmButtonModule } from "../template/confirm_button/confirm_button.module";

@NgModule({
  declarations: [Login],
  imports: [
    CommonModule,
    TranslateModule,
    LoginRoutingModule,
    GeneralInputModule,
    PasswordInputModule,
    ConfirmButtonModule
  ],
  exports: [Login]
})
export class LoginModule { }
