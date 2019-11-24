import {Component, EventEmitter, Output} from "@angular/core";

@Component({
  selector: "CookieFront",
  templateUrl: "./cookie_front.html",
  styleUrls: ["./cookie_front.scss"]
})
export class CookieFrontComponent {
  @Output() show_options: EventEmitter<boolean> = new EventEmitter();
  @Output() agree: EventEmitter<boolean> = new EventEmitter();

  emit_show_options(): void {
    this.show_options.emit(true);
  }

  emit_agree(): void {
    this.agree.emit(true);
  }
}
