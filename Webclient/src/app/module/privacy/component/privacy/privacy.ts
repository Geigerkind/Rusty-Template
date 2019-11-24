import { Component } from "@angular/core";
import { TranslationService } from "../../../../service/translation.service";
import { environment } from "src/environments/environment";

@Component({
  selector: "Privacy",
  templateUrl: "./privacy.html"
})
export class PrivacyComponent {
  privacyParams: any;

  constructor(private translationService: TranslationService) {
    this.privacyParams = environment.privacy;
  }
}
