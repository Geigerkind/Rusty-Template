import { NgModule } from "@angular/core";
import { CookieService } from "ngx-cookie-service";
import { TranslateModule } from "@ngx-translate/core";
import { NavigationBar } from "./navigation_bar";
import { ItemList } from "./item_list/item_list";
import { RouterModule } from "@angular/router";
import { CommonModule } from '@angular/common';

@NgModule({
  declarations: [
    NavigationBar,
    ItemList
  ],
  imports: [
    CommonModule,
    TranslateModule,
    RouterModule
  ],
  exports: [NavigationBar],
  providers: [CookieService],
  bootstrap: [NavigationBar]
})
export class NavigationBarModule { }
