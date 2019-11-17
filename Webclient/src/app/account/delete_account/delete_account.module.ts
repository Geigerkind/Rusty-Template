import { NgModule } from "@angular/core";
import { TranslateModule } from "@ngx-translate/core";
import { CommonModule } from "@angular/common";
import { DeleteAccountComponent } from "./delete_account";
import { PasswordInputModule } from "src/app/template/password_input/password_input.module";
import { ConfirmButtonModule } from "src/app/template/confirm_button/confirm_button.module";
import { BriefNoteModule } from "src/app/template/brief_note/brief_note.module";

@NgModule({
  declarations: [DeleteAccountComponent],
  imports: [
    CommonModule,
    TranslateModule,
    PasswordInputModule,
    ConfirmButtonModule,
    BriefNoteModule
  ],
  exports: [DeleteAccountComponent],
  bootstrap: [DeleteAccountComponent]
})
export class DeleteAccountModule { }
