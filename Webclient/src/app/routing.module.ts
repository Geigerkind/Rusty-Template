import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { AccountModule } from "./account/account.module";


const routes: Routes = [
  { path: "account", loadChildren: () => import("./account/account.module").then(m => m.AccountModule) }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
