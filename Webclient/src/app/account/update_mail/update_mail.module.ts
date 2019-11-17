import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { UpdateMailComponent } from "./update_mail";
import { PasswordInputModule } from "src/app/template/password_input/password_input.module";
import { ConfirmButtonModule } from "src/app/template/confirm_button/confirm_button.module";
import { GeneralInputModule } from "src/app/template/general_input/general_input.module";
import { BriefNoteModule } from "src/app/template/brief_note/brief_note.module";

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
export class UpdateMailModule { }
