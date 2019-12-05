import {Component, EventEmitter, Input, Output} from "@angular/core";

@Component({
    selector: "PasswordInput",
    templateUrl: "./password_input.html",
    styleUrls: ["./password_input.scss"]
})
export class PasswordInputComponent {
    @Input() placeholderKey: string;
    @Input() labelKey: string;
    @Input() name: string;

    @Output() valueChange: EventEmitter<string> = new EventEmitter<string>();
    valueData: string;
    visibility = "password";

    @Input()
    get value(): string {
        return this.valueData;
    }

    set value(newValue: string) {
        if (this.valueData !== newValue && this.valueData !== undefined) {
            console.log("Test");
            this.valueChange.emit(newValue);
        }
        this.valueData = newValue;
    }

    toggleVisibility(): void {
        if (this.visibility === "password")
            this.visibility = "text";
        else
            this.visibility = "password";
    }
}
