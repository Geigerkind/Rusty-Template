import { Component } from "@angular/core";

@Component({
  selector: "Account",
  templateUrl: "./account.html",
  styleUrls: ["./account.scss"]
})
export class Account {
  settings: Array<Array<string>> = [
    ["test1", "Entry"],
    ["test2", "Entry"],
    ["test3", "Entry"]
  ];

}
