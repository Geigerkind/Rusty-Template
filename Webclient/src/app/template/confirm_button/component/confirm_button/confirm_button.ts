import {Component, Input} from "@angular/core";

@Component({
    selector: "ConfirmButton",
    templateUrl: "./confirm_button.html",
    styleUrls: ["./confirm_button.scss"]
})
export class ConfirmButtonComponent {
    @Input() labelKey: string;
    @Input() type = "button";
}
