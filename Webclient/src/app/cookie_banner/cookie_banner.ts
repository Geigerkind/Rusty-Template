import { Component, Output, EventEmitter } from "@angular/core";
import { CookieOption } from "./material/cookie_option";
import { CookieService } from "ngx-cookie-service";
import { TranslateService } from '@ngx-translate/core';
import { Subscription } from 'rxjs';

@Component({
  selector: "CookieBanner",
  templateUrl: "./cookie_banner.html",
  styleUrls: ["./cookie_banner.scss"]
})
export class CookieBanner {
  @Output() close_banner: EventEmitter<boolean> = new EventEmitter();

  show_options = false;
  cookies_third_party: Array<CookieOption> = [];
  cookies_other: Array<CookieOption> = [];
  cookies_necessary: Array<CookieOption> = [];


  constructor(private cookieService: CookieService) {
    this.cookies_necessary.push(new CookieOption("CookieBanner.cookieDecisions.title", "CookieBanner.cookieDecisions.description", true, true));

    this.load();
  }

  set_show_options(show: boolean): void {
    this.show_options = show;
  }

  load(): void {
    if (!this.cookieService.check("cookieDecisions"))
      return;
    const cookieDecisions = JSON.parse(this.cookieService.get("cookieDecisions"));
    cookieDecisions.other.forEach((decison, i) => this.cookies_other[i].setEnabled(decison));
    cookieDecisions.third_party.forEach((decison, i) => this.cookies_third_party[i].setEnabled(decison));
  }

  save(): void {
    const cookieDecisions = {
      other: this.cookies_other.map(cookie => cookie.enabled),
      third_party: this.cookies_third_party.map(cookie => cookie.enabled)
    };

    this.cookieService.set("cookieDecisions", JSON.stringify(cookieDecisions), 30);
    this.close_banner.emit(true);
  }
}
