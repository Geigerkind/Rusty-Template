import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { AccountInformationComponent } from "./account_information";
import { CommonModule } from "@angular/common";
import { GeneralInputModule } from "src/app/template/general_input/general_input.module";

@NgModule({
  declarations: [AccountInformationComponent],
  imports: [
    CommonModule,
    TranslateModule,
    GeneralInputModule
  ],
  exports: [AccountInformationComponent],
  bootstrap: [AccountInformationComponent]
})
export class AccountInformationModule { }
