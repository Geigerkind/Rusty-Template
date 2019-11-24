import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { AccountComponent } from "./account";
import { NavigationBarComponent } from "./component/navigation_bar/navigation_bar";
import { AccountRoutingModule } from "./routing.module";
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
    AccountRoutingModule,
    CaretButtonModule
  ],
  exports: [AccountComponent]
})
export class AccountModule { }
