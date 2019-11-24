import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {CommonModule} from "@angular/common";
import {UpdateMailComponent} from "./component/update_mail/update_mail";
import {PasswordInputModule} from "src/app/template/password_input/module";
import {ConfirmButtonModule} from "src/app/template/confirm_button/module";
import {GeneralInputModule} from "src/app/template/general_input/module";
import {BriefNoteModule} from "src/app/template/brief_note/module";

@NgModule({
  declarations: [UpdateMailComponent],
  imports: [
    CommonModule,
    TranslateModule,
    GeneralInputModule,
    PasswordInputModule,
    ConfirmButtonModule,
    BriefNoteModule
  ],
  exports: [UpdateMailComponent],
  bootstrap: [UpdateMailComponent]
})
export class UpdateMailModule {
}
