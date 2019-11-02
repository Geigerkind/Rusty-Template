import { Component } from "@angular/core";
import { CookieService } from "ngx-cookie-service";

@Component({
  selector: "root",
  templateUrl: "./app.component.html",
  styleUrls: []
})
export class AppComponent {
  title = "Webclient";
  show_cookie_banner = true;

  constructor(private cookieService: CookieService) {
    // this.show_cookie_banner = !this.cookieService.check("cookieDecisions");
  }

  handle_cookie_banner_close(): void {
    this.show_cookie_banner = false;
  }
}
