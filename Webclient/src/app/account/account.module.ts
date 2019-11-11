import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { Account } from "./account";
import { NavigationBar } from "./navigation_bar/navigation_bar";
import { AccountRoutingModule } from "./routing.module";
import { CommonModule } from "@angular/common";
import { CaretButtonModule } from "../template/caret_button/caret_button.module";

@NgModule({
  declarations: [
    Account,
    NavigationBar
  ],
  imports: [
    CommonModule,
    TranslateModule,
    AccountRoutingModule,
    CaretButtonModule
  ],
  exports: [Account]
})
export class AccountModule { }
