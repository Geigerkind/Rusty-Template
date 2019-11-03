import { Component } from "@angular/core";

@Component({
  selector: "NavigationBar",
  templateUrl: "./navigation_bar.html",
  styleUrls: ["./navigation_bar.scss"]
})
export class NavigationBar {
  sampleItems = [
    ["url", "Entry 1"],
    ["url", "Entry 2"],
    ["url", "Entry 3"]
  ];

  accountItems: Array<Array<string>> = [
    ["/profile/", "NavigationBar.account.title"],
    ["/logout/", "NavigationBar.account.logout"]
  ];

}
