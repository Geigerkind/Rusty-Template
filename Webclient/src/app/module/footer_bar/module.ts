import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { FooterBar } from "./component/footer_bar/footer_bar";
import { CommonModule } from "@angular/common";
import {AppRoutingModule} from "../../routing";

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
