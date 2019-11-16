import { NgModule } from "@angular/core";
import { Routes, RouterModule } from "@angular/router";
import { Privacy } from "./privacy";

const routes: Routes = [
  { path: "", component: Privacy }
];

@NgModule({
  imports: [RouterModule.forChild(routes)],
  exports: [RouterModule]
})
export class PrivacyRoutingModule { }
