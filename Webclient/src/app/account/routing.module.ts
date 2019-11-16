import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { AccountInformation } from "./account_information/account_information";
import { Account } from "./account";

const routes: Routes = [
  { path: "", component: Account, pathMatch: "full", children: [
    { path: "", component: AccountInformation, pathMatch: "full"},
    { path: "test2", component: AccountInformation, pathMatch: "full"},
    { path: "test3", component: AccountInformation, pathMatch: "full"}
  ]}
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class AccountRoutingModule { }
