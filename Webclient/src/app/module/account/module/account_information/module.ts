import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {AccountInformationComponent} from "./component/account_information/account_information";
import {CommonModule} from "@angular/common";
import {AccountInformationRouting} from "./routing";

@NgModule({
  declarations: [AccountInformationComponent],
  imports: [
    CommonModule,
    TranslateModule,
    AccountInformationRouting
  ],
  exports: [AccountInformationComponent],
  bootstrap: [AccountInformationComponent]
})
export class AccountInformationModule {
}
