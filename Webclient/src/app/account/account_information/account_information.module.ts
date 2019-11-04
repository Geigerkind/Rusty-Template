import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { AccountInformation } from "./account_information";
import { CommonModule } from "@angular/common";
import { GeneralInputModule } from "src/app/template/general_input/general_input.module";

@NgModule({
  declarations: [AccountInformation],
  imports: [
    CommonModule,
    TranslateModule,
    GeneralInputModule
  ],
  exports: [AccountInformation],
  bootstrap: [AccountInformation]
})
export class AccountInformationModule { }
