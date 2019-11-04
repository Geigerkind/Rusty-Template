import { NgModule } from "@angular/core";

import { CookieOptions } from "./cookie_options";
import { CookieOptionRow } from "./tools/cookie_option_row/cookie_option_row";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";

@NgModule({
  declarations: [
    CookieOptions,
    CookieOptionRow
  ],
  imports: [
    CommonModule,
    TranslateModule
  ],
  exports: [CookieOptions],
  bootstrap: [CookieOptions]
})
export class CookieOptionsModule { }
