import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {CommonModule} from "@angular/common";
import {LoginRouting} from "./routing";
import {LoginComponent} from "./component/login/login";
import {GeneralInputModule} from "../../template/general_input/module";
import {PasswordInputModule} from "../../template/password_input/module";
import {ConfirmButtonModule} from "../../template/confirm_button/module";
import {LoginService} from "./service/login";
import {FormsModule} from "@angular/forms";

@NgModule({
    declarations: [LoginComponent],
    imports: [
        CommonModule,
        TranslateModule,
        LoginRouting,
        GeneralInputModule,
        PasswordInputModule,
        ConfirmButtonModule,
        FormsModule
    ],
    exports: [LoginComponent],
    providers: [LoginService]
})
export class LoginModule {
}
