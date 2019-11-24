import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";

const routes: Routes = [
  { path: "", loadChildren: () => import("./module/home/home.module").then(m => m.HomeModule) },
  { path: "account", loadChildren: () => import("./module/account/account.module").then(m => m.AccountModule) },
  { path: "login", loadChildren: () => import("./module/login/login.module").then(m => m.LoginModule) },
  { path: "sign_up", loadChildren: () => import("./module/sign_up/sign_up.module").then(m => m.SignUpModule) },
  { path: "reset_password", loadChildren: () => import("./module/reset_password/reset_password.module").then(m => m.ResetPasswordModule) },
  { path: "privacy", loadChildren: () => import("./module/privacy/privacy.module").then(m => m.PrivacyModule) },
  { path: "imprint", loadChildren: () => import("./module/imprint/imprint.module").then(m => m.ImprintModule) }
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRoutingModule { }
