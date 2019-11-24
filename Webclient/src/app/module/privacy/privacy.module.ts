import { NgModule } from "@angular/core";
import { TranslateModule, TranslateLoader } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { PrivacyComponent } from "./privacy";
import { PrivacyRoutingModule } from "./routing.module";
import { HttpClient } from "@angular/common/http";
import { TranslateHttpLoader } from "@ngx-translate/http-loader";
import { TranslationService } from "../../service/translation.service";

export function createTranslateLoader(http: HttpClient) {
  return new TranslateHttpLoader(http, "/assets/i18n/privacy/", ".json");
}

@NgModule({
  declarations: [PrivacyComponent],
  imports: [
    CommonModule,
    TranslateModule.forRoot({
      loader: {
          provide: TranslateLoader,
          useFactory: (createTranslateLoader),
          deps: [HttpClient]
      }
    }),
    PrivacyRoutingModule
  ],
  exports: [PrivacyComponent],
  providers: [
    TranslationService
  ]
})
export class PrivacyModule { }
