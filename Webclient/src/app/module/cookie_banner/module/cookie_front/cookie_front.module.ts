import { NgModule } from "@angular/core";

import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { CookieFrontComponent } from "./cookie_front";

@NgModule({
  declarations: [CookieFrontComponent],
  imports: [
    CommonModule,
    TranslateModule
  ],
  exports: [CookieFrontComponent]
})
export class CookieFrontModule { }
