import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { AccountInformationComponent } from "./account_information/account_information";
import { Account } from "./account";
import { UpdateNicknameComponent } from "./update_nickname/update_nickname";
import { UpdatePasswordComponent } from "./update_password/update_password";
import { UpdateMailComponent } from "./update_mail/update_mail";

const routes: Routes = [
  { path: "", component: Account, children: [
    { path: "", component: AccountInformationComponent},
    { path: "nickname", component: UpdateNicknameComponent},
    { path: "password", component: UpdatePasswordComponent},
    { path: "mail", component: UpdateMailComponent},
    { path: "api", component: AccountInformationComponent},
    { path: "tokens", component: AccountInformationComponent},
    { path: "delete", component: AccountInformationComponent}
  ]}
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class AccountRoutingModule { }
