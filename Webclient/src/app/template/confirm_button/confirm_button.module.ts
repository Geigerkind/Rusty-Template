import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { ConfirmButton } from "./confirm_button";

@NgModule({
  declarations: [ConfirmButton],
  imports: [
    CommonModule,
    TranslateModule
  ],
  exports: [ConfirmButton]
})
export class ConfirmButtonModule { }
