import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { UpdatePasswordComponent } from "./update_password";
import { PasswordInputModule } from "src/app/template/password_input/password_input.module";
import { ConfirmButtonModule } from "src/app/template/confirm_button/confirm_button.module";

@NgModule({
  declarations: [UpdatePasswordComponent],
  imports: [
    CommonModule,
    TranslateModule,
    PasswordInputModule,
    ConfirmButtonModule
  ],
  exports: [UpdatePasswordComponent],
  bootstrap: [UpdatePasswordComponent]
})
export class UpdatePasswordModule { }
