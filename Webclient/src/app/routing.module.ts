import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { AccountModule } from "./account/account.module";
import { Account } from "./account/account";


const routes: Routes = [
  { path: "account", component: Account, loadChildren: () => import("./account/account.module").then(m => m.AccountModule) }
];

@NgModule({
  imports: [
    RouterModule.forRoot(routes),
    AccountModule
  ],
  exports: [RouterModule]
})
export class AppRoutingModule { }
