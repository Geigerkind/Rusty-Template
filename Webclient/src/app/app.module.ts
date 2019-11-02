import { BrowserModule } from "@angular/platform-browser";
import { NgModule } from "@angular/core";

import { AppRoutingModule } from "./app-routing.module";
import { AppComponent } from "./app.component";
import { CookieBanner } from "./cookie_banner/cookie_banner";
import { CookieFront } from "./cookie_banner/cookie_front/cookie_front";
import { CookieOptions } from "./cookie_banner/cokkie_options/cookie_options";
import { CookieOptionRow } from "./cookie_banner/cokkie_options/tools/cookie_option_row/cookie_option_row";

@NgModule({
  declarations: [
    AppComponent,
    CookieBanner,
    CookieFront,
    CookieOptions,
    CookieOptionRow
  ],
  imports: [
    BrowserModule,
    AppRoutingModule
  ],
  providers: [],
  bootstrap: [AppComponent]
})
export class AppModule { }
