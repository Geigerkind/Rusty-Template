import { Component, Input, Output, EventEmitter } from "@angular/core";

@Component({
  selector: "CaretButton",
  templateUrl: "./caret_button.html",
  styleUrls: ["./caret_button.scss"]
})
export class CaretButton {
  @Input() isToggled: boolean;
  @Output() toggled: EventEmitter<boolean> = new EventEmitter();

  toggle(): void {
    this.isToggled = !this.isToggled;
    this.toggled.emit(this.isToggled);
  }
}
