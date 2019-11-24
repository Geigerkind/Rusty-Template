import { Component, Input, Output, EventEmitter } from "@angular/core";

@Component({
  selector: "ItemList",
  templateUrl: "./item_list.html",
  styleUrls: ["./item_list.scss"]
})
export class ItemList {
  @Input() items: Array<Array<string>>;
  @Output() closeMenu: EventEmitter<boolean> = new EventEmitter();

  show_sub_menu = false;

  closeNavBar(): void {
    this.show_sub_menu = false;
    this.closeMenu.emit(true);
  }
}