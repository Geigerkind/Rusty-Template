import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { AccountComponent } from "./component/account/account";
import { NavigationBarComponent } from "./component/navigation_bar/navigation_bar";
import { AccountRouting } from "./routing";
import { CommonModule } from "@angular/common";
import { CaretButtonModule } from "../../template/caret_button/caret_button.module";

@NgModule({
  declarations: [
    AccountComponent,
    NavigationBarComponent
  ],
  imports: [
    CommonModule,
    TranslateModule,
    AccountRouting,
    CaretButtonModule
  ],
  exports: [AccountComponent]
})
export class AccountModule { }
