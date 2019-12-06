import {Component, EventEmitter, Input, Output} from "@angular/core";
import {escapeRegExp} from "tslint/lib/utils";

@Component({
    selector: "GeneralInput",
    templateUrl: "./general_input.html",
    styleUrls: ["./general_input.scss"]
})
export class GeneralInputComponent {
    touched: boolean = false;
    pattern: string = ".+";

    @Input() type: string;
    @Input() placeholderKey: string;
    @Input() labelKey: string;
    @Input() required: boolean;
    @Input() minimum_length = 0;
    @Input() maximum_length = 1024;
    @Input() name: string;

    @Output() valueChange: EventEmitter<string> = new EventEmitter<string>();
    valueData = "";

    @Input()
    get value(): string {
        return this.valueData;
    }

    set value(newValue: string) {
        if (this.valueData !== newValue && this.valueData !== undefined) {
            this.touched = true;
            this.valueChange.emit(newValue);
        }
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

        if (this.forceInvalidData)
            this.pattern = "^(?!" + escapeRegExp(this.valueData) + "$).*$";
        else this.pattern = ".+";
    }
}
