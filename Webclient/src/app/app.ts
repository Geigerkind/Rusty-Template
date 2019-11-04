import { Component, OnInit } from "@angular/core";
import { CookieService } from "ngx-cookie-service";
import { TranslateService } from "@ngx-translate/core";
import { ComponentLocation } from "@wishtack/reactive-component-loader";

@Component({
  selector: "root",
  templateUrl: "./app.html",
  styleUrls: ["./app.scss"]
})
export class AppComponent implements OnInit {

  constructor(private cookieService: CookieService,
              private translateService: TranslateService) {
    this.init_translate_service();
  }

  get isCookieBannerVisible(): boolean {
    return AppComponent.show_cookie_banner;
  }

  static show_cookie_banner = false;
  title = "Webclient";
  location: ComponentLocation = null;
  cookie_banner: ComponentLocation = {
    moduleId: "cookie_banner",
    selector: "CookieBanner"
  };

  ngOnInit(): void {
    this.set_cookie_banner(!this.cookieService.check("cookieDecisions"));
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
}
