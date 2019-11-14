import { Component, OnInit } from "@angular/core";
import { CookieService } from "ngx-cookie-service";
import { TranslateService } from "@ngx-translate/core";
import { ComponentLocation } from "@wishtack/reactive-component-loader";
import { Router, NavigationEnd } from "@angular/router";
import { Subscription } from "rxjs";
import { SettingsService } from "./service/settings.service";

declare var gtag;

@Component({
  selector: "root",
  templateUrl: "./app.html",
  styleUrls: ["./app.scss"]
})
export class AppComponent implements OnInit {
  constructor(private cookieService: CookieService,
              private settingsService: SettingsService,
              private translateService: TranslateService,
              private router: Router) {
    this.init_translate_service();
    this.settingsService.subscribe("cookieDecisions", item => this.configure_google_analytics(item));
  }

  get isCookieBannerVisible(): boolean {
    return AppComponent.show_cookie_banner;
  }
  static show_cookie_banner = false;

  googleAnalyticsSubscription: Subscription;
  title = "Webclient";
  location: ComponentLocation = null;
  cookie_banner: ComponentLocation = {
    moduleId: "cookie_banner",
    selector: "CookieBanner"
  };

  ngOnInit(): void {
    this.set_cookie_banner(!this.settingsService.check("cookieDecisions"));
    this.configure_google_analytics(this.settingsService.get("cookieDecisions"));
  }

  set_cookie_banner(state: boolean): void {
    AppComponent.show_cookie_banner = state;
    if (state && this.location === null)
      this.location = this.cookie_banner;
  }

  async init_translate_service() {
    this.translateService.setDefaultLang("en");
    this.translateService.use("en");
  }

  configure_google_analytics(cookieDecisions: any): void {
    if (!cookieDecisions || !cookieDecisions.other[0]) {
      if (this.googleAnalyticsSubscription) {
        this.googleAnalyticsSubscription.unsubscribe();
      }
      return;
    }

    gtag("config", "UA-152539504-1");
    this.googleAnalyticsSubscription = this.router.events.subscribe(event => {
      if (event instanceof NavigationEnd && (window as any).ga) {
        (window as any).ga("create", "UA-152539504-1", "auto");
        (window as any).ga("set", "anonymizeIp", true);
        (window as any).ga("set", "allowAdFeatures", false);
        (window as any).ga("set", "page", event.urlAfterRedirects);
        (window as any).ga("send", "pageview");
      }
    });
  }
}
