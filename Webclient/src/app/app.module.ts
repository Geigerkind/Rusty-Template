import { BrowserModule } from "@angular/platform-browser";
import { NgModule } from "@angular/core";

import { AppRoutingModule } from "./routing.module";
import { AppComponent } from "./app";
import { CookieService } from "ngx-cookie-service";
import { CookieBannerModule } from "./module/cookie_banner/cookie_banner.module";

import {HttpClientModule, HttpClient} from "@angular/common/http";
import {TranslateHttpLoader} from "@ngx-translate/http-loader";
import { TranslateModule, TranslateLoader } from "@ngx-translate/core";
import { FooterBarModule } from "./component/footer_bar/footer_bar.module";
import { NavigationBarModule } from "./component/navigation_bar/navigation_bar.module";
import { CookieBannerComponent } from "./module/cookie_banner/cookie_banner";
import { ReactiveComponentLoaderModule } from "@wishtack/reactive-component-loader";
import { AccountModule } from "./module/account/module";
import { SettingsService } from "./service/settings.service";
import { NotificationListModule } from "./component/notification_list/notification_list.module";
import { NotificationService } from "./service/notification.service";
import { RouterLoadingBarModule } from "./component/router_loading_bar/router_loading_bar.module";
import { TranslationService } from "./service/translation.service";

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
    NotificationListModule,
    NavigationBarModule,
    RouterLoadingBarModule,
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
      loadChildren: "./module/cookie_banner/cookie_banner.module#CookieBannerModule"
    })
  ],
  providers: [
    CookieService,
    SettingsService,
    NotificationService,
    TranslationService
  ],
  bootstrap: [AppComponent],
  entryComponents: [CookieBannerComponent]
})
export class AppModule { }
