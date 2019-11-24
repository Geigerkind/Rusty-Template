import {Component} from "@angular/core";

@Component({
  selector: "NavigationBar",
  templateUrl: "./navigation_bar.html",
  styleUrls: ["./navigation_bar.scss"]
})
export class NavigationBarComponent {
  sampleItems = [
    ["url", "Entry 1"],
    ["url", "Entry 2"],
    ["url", "Entry 3"]
  ];

  accountItems: Array<Array<string>> = [
    ["/account/", "NavigationBar.account.title"],
    ["/logout/", "NavigationBar.account.logout"]
  ];

  show_item_list = false;

  toggle(): void {
    this.show_item_list = !this.show_item_list;
  }

  handleClose(): void {
    this.show_item_list = false;
  }

}
