import { NgModule } from "@angular/core";
import { CookieService } from "ngx-cookie-service";
import { BrowserModule } from "@angular/platform-browser";
import { TranslateModule } from "@ngx-translate/core";
import { RouterModule } from '@angular/router';
import { Account } from './account';

@NgModule({
  declarations: [
    Account
  ],
  imports: [
    BrowserModule,
    TranslateModule,
    RouterModule
  ],
  exports: [Account],
  providers: [CookieService],
  bootstrap: [Account]
})
export class AccountModule { }
