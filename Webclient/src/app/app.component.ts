import { Component } from "@angular/core";
import { CookieService } from "ngx-cookie-service";
import { TranslateService } from "@ngx-translate/core";
import { Observable } from "rxjs";

@Component({
  selector: "root",
  templateUrl: "./app.component.html",
  styleUrls: []
})
export class AppComponent {
  title = "Webclient";
  show_cookie_banner = true;

  constructor(private cookieService: CookieService,
              private translateService: TranslateService) {
    this.init_translate_service();
    // this.show_cookie_banner = !this.cookieService.check("cookieDecisions");
  }

  handle_cookie_banner_close(): void {
    this.show_cookie_banner = false;
  }

  async init_translate_service() {
    this.translateService.setDefaultLang("en");
    this.translateService.use("en");
  }
}
