import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";

const routes: Routes = [
  { path: "", loadChildren: () => import("./home/home.module").then(m => m.HomeModule), pathMatch: "full" },
  { path: "account", loadChildren: () => import("./account/account.module").then(m => m.AccountModule), pathMatch: "full" },
  { path: "privacy", loadChildren: () => import("./privacy/privacy.module").then(m => m.PrivacyModule), pathMatch: "full" },
  { path: "imprint", loadChildren: () => import("./imprint/imprint.module").then(m => m.ImprintModule), pathMatch: "full" }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
