import { NgModule } from "@angular/core";
import { TranslateModule, TranslateLoader } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { HttpClient } from "@angular/common/http";
import { TranslateHttpLoader } from "@ngx-translate/http-loader";
import { TranslationService } from "../service/translation.service";
import { ImprintRoutingModule } from "./routing.module";
import { Imprint } from "./imprint";

export function createTranslateLoader(http: HttpClient) {
  return new TranslateHttpLoader(http, "/assets/i18n/imprint/", ".json");
}

@NgModule({
  declarations: [Imprint],
  imports: [
    CommonModule,
    TranslateModule.forRoot({
      loader: {
          provide: TranslateLoader,
          useFactory: (createTranslateLoader),
          deps: [HttpClient]
      }
    }),
    ImprintRoutingModule
  ],
  exports: [Imprint],
  providers: [
    TranslationService
  ]
})
export class ImprintModule { }
