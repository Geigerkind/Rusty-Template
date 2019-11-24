import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { SignUpRoutingModule } from "./routing.module";
import { SignUpComponent } from "./sign_up";
import { GeneralInputModule } from "../../template/general_input/general_input.module";
import { PasswordInputModule } from "../../template/password_input/password_input.module";
import { ConfirmButtonModule } from "../../template/confirm_button/confirm_button.module";

@NgModule({
  declarations: [SignUpComponent],
  imports: [
    CommonModule,
    TranslateModule,
    SignUpRoutingModule,
    GeneralInputModule,
    PasswordInputModule,
    ConfirmButtonModule
  ],
  exports: [SignUpComponent]
})
export class SignUpModule { }
