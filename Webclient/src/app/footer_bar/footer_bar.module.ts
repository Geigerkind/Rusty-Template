import { NgModule } from "@angular/core";
import { CookieService } from "ngx-cookie-service";
import { BrowserModule } from "@angular/platform-browser";
import { TranslateModule } from "@ngx-translate/core";
import { FooterBar } from "./footer_bar";

@NgModule({
  declarations: [
    FooterBar
  ],
  imports: [
    BrowserModule,
    TranslateModule
  ],
  exports: [FooterBar],
  providers: [CookieService],
  bootstrap: [FooterBar]
})
export class FooterBarModule { }
