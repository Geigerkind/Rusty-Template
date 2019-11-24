import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { AccountInformationComponent } from "./component/account_information/account_information";
import { CommonModule } from "@angular/common";

@NgModule({
  declarations: [AccountInformationComponent],
  imports: [
    CommonModule,
    TranslateModule
  ],
  exports: [AccountInformationComponent],
  bootstrap: [AccountInformationComponent]
})
export class AccountInformationModule { }