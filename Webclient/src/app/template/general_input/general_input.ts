import { Component, Input } from "@angular/core";

@Component({
  selector: "GeneralInput",
  templateUrl: "./general_input.html",
  styleUrls: ["./general_input.scss"]
})
export class GeneralInput {
  @Input() type: string;
  @Input() placeholder: string;
  @Input() initialValue: string;

  constructor() {}

}
