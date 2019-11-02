import { Component, Input } from "@angular/core";
import { CookieOption } from "src/app/cookie_banner/material/cookie_option";

@Component({
  selector: "CookieOptionRow",
  templateUrl: "./cookie_option_row.html",
  styleUrls: ["./cookie_option_row.scss"]
})
export class CookieOptionRow {
  @Input() cookie: CookieOption;
}
