import {Component} from "@angular/core";
import {TranslationService} from "../../../../service/translation";
import {environment} from "src/environments/environment";

@Component({
    selector: "Imprint",
    templateUrl: "./imprint.html"
})
export class ImprintComponent {
    imprintParams: any;

    constructor(private translationService: TranslationService) {
        this.imprintParams = environment.imprint;
    }
}
