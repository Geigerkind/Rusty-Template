import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { ResetPassword } from "./reset_password";

const routes: Routes = [
  { path: "", component: ResetPassword }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class ResetPasswordRoutingModule { }
