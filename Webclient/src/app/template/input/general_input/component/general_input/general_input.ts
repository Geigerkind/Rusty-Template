import {Component, ElementRef, EventEmitter, Input, Output, ViewChild} from "@angular/core";
import {escapeRegExp} from "../../../../../stdlib/escapeRexExp";
import {FormFailure} from "../../../../../material/form_failure";

@Component({
    selector: "GeneralInput",
    templateUrl: "./general_input.html",
    styleUrls: ["./general_input.scss"]
})
export class GeneralInputComponent {
    touched: boolean = false;
    pattern: string;

    @ViewChild("generalInput", { static: true }) inputRef: ElementRef;
    @Input() type: string;
    @Input() placeholderKey: string;
    @Input() labelKey: string;
    @Input() required: boolean;
    @Input() maximum_length = 1024;
    @Input() min: string;
    @Input() max: string;
    @Input() name: string;

    @Output() valueChange: EventEmitter<string> = new EventEmitter<string>();
    valueData = "";

    @Input()
    get value(): string {
        return this.valueData;
    }

    set value(newValue: string) {
        if (this.valueData !== undefined && this.valueData !== newValue) {
            this.formFailure.isInvalid = false;

            this.touched = true;
            this.valueChange.emit(newValue);
        }
        this.valueData = newValue;
    }

    formFailureData: FormFailure = FormFailure.empty();
    @Input()
    get formFailure(): FormFailure {
        return this.formFailureData;
    }
    set formFailure(newValue: FormFailure) {
        this.formFailureData = newValue;
        this.updatePattern();
        this.formFailureData.subscribe(() => this.updatePattern());
    }

    constructor() {
        this.updatePattern();
    }

    updatePattern(): void {
        if (this.formFailure.isInvalid) {
            this.pattern = "^(?!" + escapeRegExp(this.valueData) + "$).*$";
        } else {
            this.pattern = undefined;
            this.formFailure.invalidityMsg = '';
            if (!!this.inputRef)
                this.inputRef.nativeElement.setCustomValidity('');
        }
    }
}
