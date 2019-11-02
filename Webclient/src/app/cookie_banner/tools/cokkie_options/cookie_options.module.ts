import { NgModule } from "@angular/core";

import { CookieOptions } from "./cookie_options";
import { CookieOptionRow } from "./tools/cookie_option_row/cookie_option_row";
import { BrowserModule } from "@angular/platform-browser";
import { TranslateModule } from "@ngx-translate/core";

@NgModule({
  declarations: [
    CookieOptions,
    CookieOptionRow
  ],
  imports: [
    BrowserModule,
    TranslateModule
  ],
  exports: [CookieOptions],
  bootstrap: [CookieOptions]
})
export class CookieOptionsModule { }
