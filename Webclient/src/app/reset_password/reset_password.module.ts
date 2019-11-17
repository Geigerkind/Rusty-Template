import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { ResetPasswordRoutingModule } from "./routing.module";
import { ResetPassword } from "./reset_password";
import { GeneralInputModule } from "../template/general_input/general_input.module";
import { ConfirmButtonModule } from "../template/confirm_button/confirm_button.module";

@NgModule({
  declarations: [ResetPassword],
  imports: [
    CommonModule,
    TranslateModule,
    ResetPasswordRoutingModule,
    GeneralInputModule,
    ConfirmButtonModule
  ],
  exports: [ResetPassword]
})
export class ResetPasswordModule { }
