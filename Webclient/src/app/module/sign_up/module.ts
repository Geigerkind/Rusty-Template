import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { SignUpRoutingModule } from "./routing";
import { SignUpComponent } from "./component/sign_up/sign_up";
import { GeneralInputModule } from "../../template/general_input/module";
import { PasswordInputModule } from "../../template/password_input/module";
import { ConfirmButtonModule } from "../../template/confirm_button/module";

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
