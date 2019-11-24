import { Component, Input } from "@angular/core";

@Component({
  selector: "PasswordInput",
  templateUrl: "./password_input.html",
  styleUrls: ["./password_input.scss"]
})
export class PasswordInputComponent {
  @Input() placeholderKey: string;
  @Input() initialValue: string;
  @Input() labelKey: string;
  @Input() name: string;

  visibility = "password";

  toggleVisibility(): void {
    if (this.visibility === "password")
      this.visibility = "text";
    else
      this.visibility = "password";
  }
}
