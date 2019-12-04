import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {CommonModule} from "@angular/common";
import {ResetPasswordRouting} from "./routing";
import {ResetPasswordComponent} from "./component/reset_password/reset_password";
import {GeneralInputModule} from "../../template/general_input/module";
import {ConfirmButtonModule} from "../../template/confirm_button/module";

@NgModule({
    declarations: [ResetPasswordComponent],
    imports: [
        CommonModule,
        TranslateModule,
        ResetPasswordRouting,
        GeneralInputModule,
        ConfirmButtonModule
    ],
    exports: [ResetPasswordComponent]
})
export class ResetPasswordModule {
}
