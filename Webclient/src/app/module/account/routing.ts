import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";
import {AccountInformationComponent} from "./module/account_information/component/account_information/account_information";
import {AccountComponent} from "./component/account/account";
import {UpdateNicknameComponent} from "./module/update_nickname/component/update_nickname/update_nickname";
import {UpdatePasswordComponent} from "./module/update_password/component/update_password/update_password";
import {UpdateMailComponent} from "./module/update_mail/component/update_mail/update_mail";
import {APITokensComponent} from "./module/api_tokens/component/api_tokens/api_tokens";
import {DeleteAccountComponent} from "./module/delete_account/component/delete_account/delete_account";

const routes: Routes = [
  {
    path: "", component: AccountComponent, children: [
      {path: "", component: AccountInformationComponent},
      {path: "nickname", component: UpdateNicknameComponent},
      {path: "password", component: UpdatePasswordComponent},
      {path: "mail", component: UpdateMailComponent},
      {path: "api", component: APITokensComponent},
      {path: "delete", component: DeleteAccountComponent}
    ]
  }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class AccountRouting {
}
