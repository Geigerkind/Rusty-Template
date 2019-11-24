import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { ResetPasswordComponent } from "./reset_password";

const routes: Routes = [
  { path: "", component: ResetPasswordComponent }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class ResetPasswordRoutingModule { }
