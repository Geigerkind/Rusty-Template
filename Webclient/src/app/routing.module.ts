import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";

const routes: Routes = [
  { path: "", loadChildren: () => import("./home/home.module").then(m => m.HomeModule) },
  { path: "account", loadChildren: () => import("./account/account.module").then(m => m.AccountModule) },
  { path: "login", loadChildren: () => import("./login/login.module").then(m => m.LoginModule) },
  { path: "sign_up", loadChildren: () => import("./sign_up/sign_up.module").then(m => m.SignUpModule) },
  { path: "reset_password", loadChildren: () => import("./reset_password/reset_password.module").then(m => m.ResetPasswordModule) },
  { path: "privacy", loadChildren: () => import("./privacy/privacy.module").then(m => m.PrivacyModule) },
  { path: "imprint", loadChildren: () => import("./imprint/imprint.module").then(m => m.ImprintModule) }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
