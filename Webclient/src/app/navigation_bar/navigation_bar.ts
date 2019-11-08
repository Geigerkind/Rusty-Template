import { Component } from "@angular/core";
import { HostListener } from "@angular/core";

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
    ["/account/", "NavigationBar.account.title"],
    ["/logout/", "NavigationBar.account.logout"]
  ];

  screenWidth = 0;
  show_item_list = false;

  constructor() {
    this.getScreenSize();
  }

  @HostListener("window:resize", ["$event"])
  getScreenSize(event?): void {
    this.screenWidth = window.innerWidth;
  }

  toggle(): void {
    this.show_item_list = !this.show_item_list;
  }

  getItemListVisibility(): string {
    if (this.screenWidth > 760)
      return "flex";
    if (this.show_item_list)
      return "block";
    return "none";
  }

  handleClose(): void {
    this.show_item_list = false;
  }

}
