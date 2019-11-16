import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { AccountInformationComponent } from "./account_information/account_information";
import { Account } from "./account";

const routes: Routes = [
  { path: "", component: Account, pathMatch: "full", children: [
    { path: "", component: AccountInformationComponent, pathMatch: "full"},
    { path: "test2", component: AccountInformationComponent, pathMatch: "full"},
    { path: "test3", component: AccountInformationComponent, pathMatch: "full"}
  ]}
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class AccountRoutingModule { }
