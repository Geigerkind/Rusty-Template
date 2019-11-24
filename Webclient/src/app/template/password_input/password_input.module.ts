import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { PasswordInputComponent } from "./password_input";
import { GeneralInputModule } from "../general_input/general_input.module";

@NgModule({
  declarations: [PasswordInputComponent],
  imports: [
    CommonModule,
    TranslateModule,
    GeneralInputModule
  ],
  exports: [PasswordInputComponent]
})
export class PasswordInputModule { }
