import { Component, Input } from "@angular/core";

@Component({
  selector: "ItemList",
  templateUrl: "./item_list.html",
  styleUrls: ["./item_list.scss"]
})
export class ItemList {
  @Input() items: Array<Array<string>>;
  @Input() screenWidth: number;

  show_sub_menu = false;
  toggle(): void {
    this.show_sub_menu = !this.show_sub_menu;
  }

  getSubListVisibility(): string {
    if (this.screenWidth > 760 || !this.show_sub_menu)
      return "none";
    return "block";
  }
}
