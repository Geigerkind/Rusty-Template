import { Component } from "@angular/core";
import { CookieOption } from './material/cookie_option';

@Component({
  selector: "CookieBanner",
  templateUrl: "./cookie_banner.html",
  styleUrls: ["./cookie_banner.scss"]
})
export class CookieBanner {
  show_options = true;

  cookies_third_party: Array<CookieOption> = [];
  cookies_other: Array<CookieOption> = [];
  cookies_necessary: Array<CookieOption> = [];

  constructor() {
    this.cookies_third_party.push(new CookieOption("Lorem ipsum dolor sit", "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren", true, false));
    this.cookies_third_party.push(new CookieOption("Lorem ipsum dolor sit", "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren", true, false));

    this.cookies_other.push(new CookieOption("Lorem ipsum dolor sit", "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren", true, false));
    this.cookies_other.push(new CookieOption("Lorem ipsum dolor sit", "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren", true, false));

    this.cookies_necessary.push(new CookieOption("Lorem ipsum dolor sit", "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren", true, true));
    this.cookies_necessary.push(new CookieOption("Lorem ipsum dolor sit", "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren", true, true));
  }

  set_show_options(show: boolean): void {
    this.show_options = show;
  }
}
