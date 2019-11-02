import { NgModule } from "@angular/core";

import { CookieBanner } from "./cookie_banner";
import { CookieFront } from "./tools/cookie_front/cookie_front";
import { CookieService } from "ngx-cookie-service";
import { BrowserModule } from '@angular/platform-browser';
import { CookieOptionsModule } from './tools/cokkie_options/cookie_options.module';

@NgModule({
  declarations: [
    CookieBanner,
    CookieFront
  ],
  imports: [
    BrowserModule,
    CookieOptionsModule
  ],
  exports: [CookieBanner],
  providers: [CookieService],
  bootstrap: [CookieBanner]
})
export class CookieBannerModule { }
