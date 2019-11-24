import { NgModule } from "@angular/core";

import { CookieBannerComponent } from "./cookie_banner";
import { CookieOptionsModule } from "./module/cokkie_options/cookie_options.module";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { CookieFrontModule } from "./module/cookie_front/cookie_front.module";

@NgModule({
  declarations: [CookieBannerComponent],
  imports: [
    CommonModule,
    CookieFrontModule,
    CookieOptionsModule,
    TranslateModule
  ],
  exports: [CookieBannerComponent],
  bootstrap: [CookieBannerComponent]
})
export class CookieBannerModule { }
