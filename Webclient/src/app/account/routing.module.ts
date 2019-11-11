import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { AccountInformation } from "./account_information/account_information";
import { AccountInformationModule } from "./account_information/account_information.module";
import { Account } from "./account";

const routes: Routes = [
  { path: "", component: Account, pathMatch: "full", children: [
    { path: "", component: AccountInformation},
    { path: "test2", component: AccountInformation},
    { path: "test3", component: AccountInformation}
  ]}
];

@NgModule({
  imports: [
    RouterModule.forChild(routes),
    AccountInformationModule
  ],
  exports: [RouterModule]
})
export class AccountRoutingModule { }
