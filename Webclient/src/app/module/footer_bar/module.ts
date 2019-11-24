import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { FooterBarComponent } from "./component/footer_bar/footer_bar";
import { CommonModule } from "@angular/common";
import {AppRoutingModule} from "../../routing";

@NgModule({
  declarations: [FooterBarComponent],
  imports: [
    CommonModule,
    TranslateModule,
    AppRoutingModule
  ],
  exports: [FooterBarComponent]
})
export class FooterBarModule { }
