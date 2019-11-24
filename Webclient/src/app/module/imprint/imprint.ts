import { Component } from "@angular/core";
import { TranslationService } from "../../service/translation.service";
import { environment } from "src/environments/environment";

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
