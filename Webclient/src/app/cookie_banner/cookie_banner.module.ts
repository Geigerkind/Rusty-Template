import { NgModule } from "@angular/core";

import { CookieBanner } from "./cookie_banner";
import { CookieFront } from "./tools/cookie_front/cookie_front";
import { CookieService } from "ngx-cookie-service";
import { CookieOptionsModule } from "./tools/cokkie_options/cookie_options.module";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";

@NgModule({
  declarations: [
    CookieBanner,
    CookieFront
  ],
  imports: [
    CommonModule,
    CookieOptionsModule,
    TranslateModule
  ],
  exports: [CookieBanner],
  providers: [CookieService],
  bootstrap: [CookieBanner]
})
export class CookieBannerModule { }
