import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {CommonModule} from "@angular/common";
import {DeleteAccountComponent} from "./component/delete_account/delete_account";
import {PasswordInputModule} from "src/app/template/password_input/module";
import {ConfirmButtonModule} from "src/app/template/confirm_button/module";
import {BriefNoteModule} from "src/app/template/brief_note/module";
import {DeleteAccountRouting} from "./routing";

@NgModule({
    declarations: [DeleteAccountComponent],
    imports: [
        CommonModule,
        TranslateModule,
        PasswordInputModule,
        ConfirmButtonModule,
        BriefNoteModule,
        DeleteAccountRouting
    ],
    exports: [DeleteAccountComponent],
    bootstrap: [DeleteAccountComponent]
})
export class DeleteAccountModule {
}
