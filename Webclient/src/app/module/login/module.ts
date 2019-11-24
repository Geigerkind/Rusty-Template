import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { LoginRoutingModule } from "./routing";
import { LoginComponent } from "./component/login/login";
import { GeneralInputModule } from "../../template/general_input/module";
import { PasswordInputModule } from "../../template/password_input/module";
import { ConfirmButtonModule } from "../../template/confirm_button/module";

@NgModule({
  declarations: [LoginComponent],
  imports: [
    CommonModule,
    TranslateModule,
    LoginRoutingModule,
    GeneralInputModule,
    PasswordInputModule,
    ConfirmButtonModule
  ],
  exports: [LoginComponent]
})
export class LoginModule { }
