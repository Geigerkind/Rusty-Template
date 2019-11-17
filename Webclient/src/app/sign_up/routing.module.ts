import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { SignUp } from "./sign_up";

const routes: Routes = [
  { path: "", component: SignUp }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class SignUpRoutingModule { }
