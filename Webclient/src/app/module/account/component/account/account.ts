import { Component } from "@angular/core";

@Component({
  selector: "Account",
  templateUrl: "./account.html",
  styleUrls: ["./account.scss"]
})
export class AccountComponent {
  settings: Array<Array<string>> = [
    ["./", "Account.navBar.entries.overview"],
    ["nickname", "Account.navBar.entries.nickname"],
    ["password", "Account.navBar.entries.password"],
    ["mail", "Account.navBar.entries.mail"],
    ["api", "Account.navBar.entries.api"],
    ["delete", "Account.navBar.entries.delete"],
  ];
}