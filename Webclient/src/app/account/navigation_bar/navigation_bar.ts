import { Component, Input } from "@angular/core";
import { HostListener } from "@angular/core";

@Component({
  selector: "NavigationBar",
  templateUrl: "./navigation_bar.html",
  styleUrls: ["./navigation_bar.scss"]
})
export class NavigationBar {
  @Input() itemList: Array<Array<string>>;
  @Input() screenWidth: number;
  show_sub_menu = false;

  getSubListVisibility(): string {
    if (this.screenWidth <= 640 && !this.show_sub_menu)
      return "none";
    return "block";
  }
}
