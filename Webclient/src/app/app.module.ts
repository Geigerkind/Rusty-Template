import { BrowserModule } from "@angular/platform-browser";
import { NgModule } from "@angular/core";

import { AppRoutingModule } from "./routing.module";
import { AppComponent } from "./app";
import { CookieService } from "ngx-cookie-service";
import { CookieBannerModule } from "./cookie_banner/cookie_banner.module";

import {HttpClientModule, HttpClient} from "@angular/common/http";
import {TranslateHttpLoader} from "@ngx-translate/http-loader";
import { TranslateModule, TranslateLoader } from "@ngx-translate/core";
import { FooterBarModule } from "./footer_bar/footer_bar.module";
import { NavigationBarModule } from "./navigation_bar/navigation_bar.module";
import { CookieBanner } from "./cookie_banner/cookie_banner";
import { ReactiveComponentLoaderModule } from "@wishtack/reactive-component-loader";
import { AccountModule } from './account/account.module';

export function createTranslateLoader(http: HttpClient) {
  return new TranslateHttpLoader(http, "./assets/i18n/", ".json");
}

@NgModule({
  declarations: [
    AppComponent
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    CookieBannerModule,
    AccountModule,
    NavigationBarModule,
    FooterBarModule,
    HttpClientModule,
    TranslateModule.forRoot({
        loader: {
            provide: TranslateLoader,
            useFactory: (createTranslateLoader),
            deps: [HttpClient]
        }
    }),
    ReactiveComponentLoaderModule.forRoot(),
    ReactiveComponentLoaderModule.withModule({
      moduleId: "cookie_banner",
      loadChildren: "./cookie_banner/cookie_banner.module#CookieBannerModule"
    })
  ],
  providers: [CookieService],
  bootstrap: [AppComponent],
  entryComponents: [CookieBanner]
})
export class AppModule { }
