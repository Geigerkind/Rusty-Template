import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { NavigationBarComponent } from "./component/navigation_bar/navigation_bar";
import { ItemList } from "./component/item_list/item_list";
import { RouterModule } from "@angular/router";
import { CommonModule } from "@angular/common";
import { CaretButtonModule } from "../../template/caret_button/module";

@NgModule({
  declarations: [
    NavigationBarComponent,
    ItemList
  ],
  imports: [
    CommonModule,
    TranslateModule,
    RouterModule,
    CaretButtonModule
  ],
  exports: [NavigationBarComponent]
})
export class NavigationBarModule { }
