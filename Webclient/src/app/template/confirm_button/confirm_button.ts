import { Component, Input } from "@angular/core";

@Component({
  selector: "ConfirmButton",
  templateUrl: "./confirm_button.html",
  styleUrls: ["./confirm_button.scss"]
})
export class ConfirmButton {
  @Input() labelKey: string;
}
