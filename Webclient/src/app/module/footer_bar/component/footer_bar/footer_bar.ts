import { Component, Output, EventEmitter } from "@angular/core";
import { environment } from "src/environments/environment";

@Component({
  selector: "FooterBar",
  templateUrl: "./footer_bar.html",
  styleUrls: ["./footer_bar.scss"]
})
export class FooterBar {
  @Output() consent: EventEmitter<boolean> = new EventEmitter();

  copyRightArguments: any = {company: environment.company, year: (new Date()).getFullYear().toString() };

  show_consent(): void {
    this.consent.emit(true);
  }
}