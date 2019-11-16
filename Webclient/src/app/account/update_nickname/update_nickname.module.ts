import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { UpdateNicknameComponent } from "./update_nickname";

@NgModule({
  declarations: [UpdateNicknameComponent],
  imports: [
    CommonModule,
    TranslateModule
  ],
  exports: [UpdateNicknameComponent],
  bootstrap: [UpdateNicknameComponent]
})
export class UpdateNicknameModule { }
