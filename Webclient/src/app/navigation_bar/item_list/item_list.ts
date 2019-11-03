import { Component, Input } from "@angular/core";

@Component({
  selector: "ItemList",
  templateUrl: "./item_list.html",
  styleUrls: ["./item_list.scss"]
})
export class ItemList {
  @Input() items: Array<Array<string>>;
}
