import {NgModule} from "@angular/core";
import {TranslateModule} from "@ngx-translate/core";
import {CommonModule} from "@angular/common";
import {ConfirmButtonModule} from "src/app/template/button/confirm_button/module";
import {GeneralInputModule} from "src/app/template/input/general_input/module";
import {APITokensComponent} from "./component/api_tokens/api_tokens";
import {APITokensRouting} from "./routing";

@NgModule({
    declarations: [APITokensComponent],
    imports: [
        CommonModule,
        TranslateModule,
        GeneralInputModule,
        ConfirmButtonModule,
        APITokensRouting
    ],
    exports: [APITokensComponent],
    bootstrap: [APITokensComponent]
})
export class APITokensModule {
}
