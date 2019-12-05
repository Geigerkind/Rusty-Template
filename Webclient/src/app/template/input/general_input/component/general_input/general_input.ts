import {Component, EventEmitter, Input, Output} from "@angular/core";

@Component({
    selector: "GeneralInput",
    templateUrl: "./general_input.html",
    styleUrls: ["./general_input.scss"]
})
export class GeneralInputComponent {
    touched: boolean = false;

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
}
