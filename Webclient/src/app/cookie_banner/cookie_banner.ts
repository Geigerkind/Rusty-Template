import { Component, Output, EventEmitter } from "@angular/core";
import { CookieOption } from "./material/cookie_option";
import { SettingsService } from "../service/settings.service";
import { NotificationService } from "../service/notification.service";
import { Severity } from "../domainvalue/severity";

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


  constructor(private settingsService: SettingsService,
              private notificationService: NotificationService) {
    this.cookies_necessary.push(new CookieOption("CookieBanner.cookieDecisions.title", "CookieBanner.cookieDecisions.description", true, true));
    this.cookies_necessary.push(new CookieOption("CookieBanner.pwa_prompt.title", "CookieBanner.pwa_prompt.description", true, true));
    this.cookies_other.push(new CookieOption("CookieBanner.googleAnalytics.title", "CookieBanner.googleAnalytics.description", true, false));

    this.load();
  }

  set_show_options(show: boolean): void {
    this.show_options = show;
  }

  load(): void {
    if (!this.settingsService.check("cookieDecisions"))
      return;
    const cookieDecisions = this.settingsService.get("cookieDecisions");
    cookieDecisions.other.forEach((decison, i) => this.cookies_other[i].setEnabled(decison));
    cookieDecisions.third_party.forEach((decison, i) => this.cookies_third_party[i].setEnabled(decison));
  }

  save(): void {
    const cookieDecisions = {
      other: this.cookies_other.map(cookie => cookie.enabled),
      third_party: this.cookies_third_party.map(cookie => cookie.enabled)
    };

    this.settingsService.set("cookieDecisions", cookieDecisions, 30);
    this.notificationService.notify(Severity.Success, "CookieBanner.notification.saved");
    this.close_banner.emit(false);
  }
}
