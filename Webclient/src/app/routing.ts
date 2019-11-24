import {NgModule} from "@angular/core";
import {RouterModule, Routes} from "@angular/router";

const routes: Routes = [
  {path: "", loadChildren: () => import("./module/home/module").then(m => m.HomeModule)},
  {path: "account", loadChildren: () => import("./module/account/module").then(m => m.AccountModule)},
  {path: "login", loadChildren: () => import("./module/login/module").then(m => m.LoginModule)},
  {path: "sign_up", loadChildren: () => import("./module/sign_up/module").then(m => m.SignUpModule)},
  {
    path: "reset_password",
    loadChildren: () => import("./module/reset_password/module").then(m => m.ResetPasswordModule)
  },
  {path: "privacy", loadChildren: () => import("./module/privacy/module").then(m => m.PrivacyModule)},
  {path: "imprint", loadChildren: () => import("./module/imprint/module").then(m => m.ImprintModule)}
];

@NgModule({
  imports: [RouterModule.forRoot(routes)],
  exports: [RouterModule]
})
export class AppRouting {
}
