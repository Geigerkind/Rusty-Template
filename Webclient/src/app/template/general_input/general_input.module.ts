import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { GeneralInput } from "./general_input";

@NgModule({
  declarations: [
    GeneralInput
  ],
  imports: [
    CommonModule,
    TranslateModule
  ],
  exports: [GeneralInput],
  bootstrap: [GeneralInput]
})
export class GeneralInputModule { }
