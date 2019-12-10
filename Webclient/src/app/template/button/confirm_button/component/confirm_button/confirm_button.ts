import {Component, EventEmitter, Input, Output} from "@angular/core";

@Component({
    selector: "ConfirmButton",
    templateUrl: "./confirm_button.html",
    styleUrls: ["./confirm_button.scss"]
})
export class ConfirmButtonComponent {
    @Input() labelKey: string;
    @Input() type = "button";
    @Input() disabled = false;

    @Output() clicked: EventEmitter<boolean> = new EventEmitter();

    handleClick(): void {
        if (!this.disabled)
            this.clicked.emit(true);
    }
}
