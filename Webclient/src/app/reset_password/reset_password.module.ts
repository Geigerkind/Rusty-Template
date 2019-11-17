import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { ResetPasswordRoutingModule } from "./routing.module";
import { ResetPassword } from "./reset_password";

@NgModule({
  declarations: [ResetPassword],
  imports: [
    CommonModule,
    TranslateModule,
    ResetPasswordRoutingModule
  ],
  exports: [ResetPassword]
})
export class ResetPasswordModule { }
