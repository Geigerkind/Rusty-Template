import {Component, EventEmitter, Input, Output} from "@angular/core";
import {escapeRegExp} from "tslint/lib/utils";

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
    @Input()
    get value(): string {
        return this.valueData;
    }

    set value(newValue: string) {
        if (this.valueData !== newValue && this.valueData !== undefined)
            this.valueChange.emit(newValue);
        this.valueData = newValue;
    }

    @Output() forceInvalidChange: EventEmitter<boolean> = new EventEmitter<boolean>();
    forceInvalidData: boolean = false;

    @Input()
    get forceInvalid(): boolean {
        return this.forceInvalidData;
    }

    set forceInvalid(newValue: boolean) {
        if (this.forceInvalidData !== newValue)
            this.forceInvalidChange.emit(true);
        this.forceInvalidData = newValue;
    }

    visibility = "password";
    toggleVisibility(): void {
        if (this.visibility === "password")
            this.visibility = "text";
        else
            this.visibility = "password";
    }
}
