import {Component, EventEmitter, Output} from "@angular/core";
import {environment} from "src/environments/environment";

@Component({
    selector: "FooterBar",
    templateUrl: "./footer_bar.html",
    styleUrls: ["./footer_bar.scss"]
})
export class FooterBarComponent {
    @Output() consent: EventEmitter<boolean> = new EventEmitter();

    private copyRightArguments: any = {company: environment.company, year: (new Date()).getFullYear().toString()};

    private show_consent(): void {
        this.consent.emit(true);
    }
}
