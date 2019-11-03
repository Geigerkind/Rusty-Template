import { NgModule } from "@angular/core";
import { CookieService } from "ngx-cookie-service";
import { BrowserModule } from "@angular/platform-browser";
import { TranslateModule } from "@ngx-translate/core";
import { NavigationBar } from "./navigation_bar";
import { ItemList } from "./item_list/item_list";
import { RouterModule } from "@angular/router";

@NgModule({
  declarations: [
    NavigationBar,
    ItemList
  ],
  imports: [
    BrowserModule,
    TranslateModule,
    RouterModule
  ],
  exports: [NavigationBar],
  providers: [CookieService],
  bootstrap: [NavigationBar]
})
export class NavigationBarModule { }