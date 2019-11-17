import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { PasswordInput } from "./password_input";
import { GeneralInputModule } from "../general_input/general_input.module";

@NgModule({
  declarations: [PasswordInput],
  imports: [
    CommonModule,
    TranslateModule,
    GeneralInputModule
  ],
  exports: [PasswordInput]
})
export class PasswordInputModule { }
