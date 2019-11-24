import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {CommonModule} from "@angular/common";
import {UpdateNicknameComponent} from "./component/update_nickname/update_nickname";
import {GeneralInputModule} from "src/app/template/general_input/module";
import {ConfirmButtonModule} from "src/app/template/confirm_button/module";
import {UpdateNicknameRouting} from "./routing";

@NgModule({
  declarations: [UpdateNicknameComponent],
  imports: [
    CommonModule,
    TranslateModule,
    GeneralInputModule,
    ConfirmButtonModule,
    UpdateNicknameRouting
  ],
  exports: [UpdateNicknameComponent],
  bootstrap: [UpdateNicknameComponent]
})
export class UpdateNicknameModule {
}
