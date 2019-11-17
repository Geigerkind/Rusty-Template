import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { AccountInformationComponent } from "./account_information/account_information";
import { Account } from "./account";
import { UpdateNicknameComponent } from "./update_nickname/update_nickname";
import { UpdatePasswordComponent } from "./update_password/update_password";
import { UpdateMailComponent } from "./update_mail/update_mail";
import { APITokensComponent } from "./api_tokens/api_tokens";
import { DeleteAccountComponent } from "./delete_account/delete_account";

const routes: Routes = [
  { path: "", component: Account, children: [
    { path: "", component: AccountInformationComponent},
    { path: "nickname", component: UpdateNicknameComponent},
    { path: "password", component: UpdatePasswordComponent},
    { path: "mail", component: UpdateMailComponent},
    { path: "api", component: APITokensComponent},
    { path: "delete", component: DeleteAccountComponent}
  ]}
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class AccountRoutingModule { }
