import { Component, Input } from "@angular/core";
import { HostListener } from "@angular/core";

@Component({
  selector: "NavigationBar",
  templateUrl: "./navigation_bar.html",
  styleUrls: ["./navigation_bar.scss"]
})
export class NavigationBar {
  @Input() itemList: Array<Array<string>>;
  show_sub_menu = false;
}
