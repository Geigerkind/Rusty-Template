import { NgModule } from "@angular/core";

import { CookieOptionsComponent } from "./cookie_options";
import { CookieOptionRowComponent } from "./component/cookie_option_row/cookie_option_row";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";

@NgModule({
  declarations: [
    CookieOptionsComponent,
    CookieOptionRowComponent
  ],
  imports: [
    CommonModule,
    TranslateModule
  ],
  exports: [CookieOptionsComponent]
})
export class CookieOptionsModule { }
