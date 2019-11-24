import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { FooterBar } from "./footer_bar";
import { CommonModule } from "@angular/common";
import { AppRoutingModule } from "../../routing.module";

@NgModule({
  declarations: [FooterBar],
  imports: [
    CommonModule,
    TranslateModule,
    AppRoutingModule
  ],
  exports: [FooterBar]
})
export class FooterBarModule { }
