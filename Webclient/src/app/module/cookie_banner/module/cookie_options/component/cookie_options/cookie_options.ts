import {Component, EventEmitter, Input, Output} from "@angular/core";
import {CookieOption} from "../../../../material/cookie_option";

@Component({
    selector: "CookieOptions",
    templateUrl: "./cookie_options.html",
    styleUrls: ["./cookie_options.scss"]
})
export class CookieOptionsComponent {
    @Output() show_front: EventEmitter<boolean> = new EventEmitter();
    @Output() save: EventEmitter<boolean> = new EventEmitter();

    @Input() cookies_third_party: Array<CookieOption>;
    @Input() cookies_other: Array<CookieOption>;
    @Input() cookies_necessary: Array<CookieOption>;

    private emit_show_front(): void {
        this.show_front.emit(true);
    }

    private emit_save(): void {
        this.save.emit(true);
    }

    private reject_all(): void {
        this.cookies_other.forEach(cookie => cookie.setEnabled(false));
        this.cookies_third_party.forEach(cookie => cookie.setEnabled(false));
    }

    private accept_all(options): void {
        options.forEach(cookie => cookie.setEnabled(true));
    }
}
