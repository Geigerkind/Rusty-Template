import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { FooterBar } from "./footer_bar";
import { CommonModule } from "@angular/common";

@NgModule({
  declarations: [FooterBar],
  imports: [
    CommonModule,
    TranslateModule
  ],
  exports: [FooterBar]
})
export class FooterBarModule { }
