import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { SignUpRoutingModule } from "./routing.module";
import { SignUp } from "./sign_up";
import { GeneralInputModule } from "../template/general_input/general_input.module";
import { PasswordInputModule } from "../template/password_input/password_input.module";
import { ConfirmButtonModule } from "../template/confirm_button/confirm_button.module";

@NgModule({
  declarations: [SignUp],
  imports: [
    CommonModule,
    TranslateModule,
    SignUpRoutingModule,
    GeneralInputModule,
    PasswordInputModule,
    ConfirmButtonModule
  ],
  exports: [SignUp]
})
export class SignUpModule { }
