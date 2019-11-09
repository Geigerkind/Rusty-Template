import { Component, Input, Output, EventEmitter } from "@angular/core";

@Component({
  selector: "ItemList",
  templateUrl: "./item_list.html",
  styleUrls: ["./item_list.scss"]
})
export class ItemList {
  @Input() items: Array<Array<string>>;
  @Input() screenWidth: number;
  @Output() closeMenu: EventEmitter<boolean> = new EventEmitter();

  show_sub_menu: boolean = false;

  getSubListVisibility(): string {
    if (this.screenWidth > 760 || !this.show_sub_menu)
      return "none";
    return "block";
  }

  closeNavBar(): void {
    this.show_sub_menu = false;
    this.closeMenu.emit(true);
  }
}
